pub(crate) mod applications;
pub(crate) mod auth;
pub(crate) mod error;
pub(crate) mod graphql;
pub(crate) mod guards;
pub(crate) mod schema;
pub(crate) mod util;

use std::{future::Future, pin::Pin, sync::Arc};

use async_graphql::{EmptySubscription, Schema};
use auth::authenticate_request;
use graphql::{mutations::Mutation, queries::Query};
use lambda_http::{http::Method, Body, Error, Request, Response, Service};
use sea_orm::{Database, DatabaseConnection};
pub use util::variables::SECRET_VARIABLES;

#[derive(Clone)]
pub struct App {
    schema: Arc<Schema<Query, Mutation, EmptySubscription>>,
    db: DatabaseConnection,
}

impl App {
    pub async fn new(test_database_url: Option<String>) -> Result<Self, Error> {
        // setup tracking for logs
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            // disable printing the name of the module in every log line.
            .with_target(false)
            .log_internal_errors(true)
            // disabling time is handy because CloudWatch will add the ingestion time.
            .without_time()
            .try_init()
            .ok();

        Ok(Self {
            schema: Arc::new(Schema::new(
                Query::default(),
                Mutation::default(),
                EmptySubscription,
            )),
            db: Database::connect(match test_database_url {
                Some(url) => url,
                None => SECRET_VARIABLES
                    .database_url
                    .clone()
                    .expect("DATABASE_URL not set"),
            })
            .await?,
        })
    }

    pub async fn respond(&self, event: Request) -> Result<Response<Body>, Error> {
        tracing::info!("Handling {} request...", event.method());
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
            serde_json::from_str::<async_graphql::Request>(body)?.data(self.db.clone());

        match authenticate_request(&self.db, event).await {
            Ok(Some((user, scopes))) => graphql_request = graphql_request.data(user).data(scopes),
            Ok(None) => {}
            Err(e) => {
                return Ok(async_graphql::Response::from_errors(vec![
                    e.into_server_error(async_graphql::Pos { line: 0, column: 0 })
                ]))
            }
        };
        let res = self.schema.execute(graphql_request).await;

        Ok(res)
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
