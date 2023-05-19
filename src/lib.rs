pub(crate) mod db_schema;
pub(crate) mod error;
pub(crate) mod guards;
pub(crate) mod misc;
pub(crate) mod mutation;
pub(crate) mod query;
pub(crate) mod stripe;
pub(crate) mod tls;
pub(crate) mod types;
pub(crate) mod variables;

use std::{future::Future, ops::Deref, pin::Pin, sync::Arc};

use async_graphql::{futures_util::future::BoxFuture, EmptySubscription, Schema};
use diesel::{ConnectionError, ConnectionResult};
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use lambda_http::{http::Method, Body, Error, Request, Response, Service};
use openai::set_key;
use types::user::User;
use variables::init_non_secret_variables;
#[derive(Clone)]
pub struct App {
    schema: Arc<Schema<query::Query, mutation::Mutation, EmptySubscription>>,
    diesel: DieselPool,
}

#[derive(Clone)]
struct DieselPool(Arc<Pool<AsyncPgConnection>>);

impl Deref for DieselPool {
    type Target = Pool<AsyncPgConnection>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn establish_connection(url: &str) -> BoxFuture<ConnectionResult<AsyncPgConnection>> {
    Box::pin(async move {
        let connector = tokio_postgres_rustls::MakeRustlsConnect::new(tls::build_client_config());

        let (client, connection) = tokio_postgres::connect(url, connector)
            .await
            .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        AsyncPgConnection::try_from(client).await
    })
}

pub async fn create_pool(db_url: &str) -> Result<Pool<AsyncPgConnection>, anyhow::Error> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_setup(
        db_url,
        establish_connection,
    );
    Ok(Pool::builder(config).build()?)
}

impl App {
    pub async fn new() -> Result<Self, Error> {
        // There may or may not be a .env file, so we ignore the error.
        dotenv::dotenv().ok();
        init_non_secret_variables();

        set_key(dotenv::var("OPENAI_KEY").unwrap());

        let postgrest_url: String =
            dotenv::var("DATABASE_URL").expect("DATABASE_URL not set in .env");

        let pool = create_pool(&postgrest_url).await?;

        Ok(Self {
            schema: Arc::new(Schema::new(
                query::Query::default(),
                mutation::Mutation::default(),
                EmptySubscription,
            )),
            diesel: DieselPool(Arc::new(pool)),
        })
    }

    pub async fn respond(&self, event: Request) -> Result<Response<Body>, Error> {
        println!("Handling {} request...", event.method());
        let response = Response::builder();

        match *event.method() {
            Method::OPTIONS => self.handle_options().await,
            Method::POST => self.handle_post(event).await,
            _ => response
                .status(405)
                .header("Allow", "POST, OPTIONS")
                .body("405: Method not allowed - use POST instead.".into())
                .map_err(Error::from),
        }
        .map_err(Error::from)
    }

    async fn handle_options(&self) -> Result<Response<Body>, Error> {
        let response = Response::builder();

        response
            .status(200)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "POST")
            .header("Access-Control-Allow-Headers", "*")
            .body(Body::Empty)
            .map_err(Error::from)
    }

    async fn graph_endpoint(
        &self,
        event: Request,
    ) -> Result<async_graphql::Response, anyhow::Error> {
        let body = std::str::from_utf8(event.body())?;
        let mut graphql_request =
            serde_json::from_str::<async_graphql::Request>(body)?.data(self.diesel.clone());

        // get token from header
        let token = event
            .headers()
            .get("Authorization")
            .map(|v| v.to_str().map(|s| s.to_string()))
            .transpose()?;

        if let Some(token) = token {
            match User::authenticate_from_token(&self.diesel, token).await {
                Ok(user) => graphql_request = graphql_request.data(user),
                Err(e) => return Err(e),
            };
        };

        Ok(self.schema.execute(graphql_request).await)
    }

    async fn handle_post(&self, event: Request) -> Result<Response<Body>, Error> {
        let response = Response::builder();

        let graphql_response = self.graph_endpoint(event).await?;

        let json = serde_json::to_string(&graphql_response)?;

        response
            .status(200)
            .header("content-type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "POST")
            .header("Access-Control-Allow-Headers", "*")
            .body(json.into())
            .map_err(Error::from)
    }
}

impl Service<Request> for App {
    type Response = Response<Body>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, event: Request) -> Self::Future {
        let app = self.clone();

        Box::pin(async move { app.respond(event).await })
    }
}
