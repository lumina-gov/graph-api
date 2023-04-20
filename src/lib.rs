pub mod error;
pub mod graph;
pub mod models;
pub mod stripe;

use std::{sync::Arc, future::Future, pin::Pin};

use dotenv;
pub use graph::{
    context::GeneralContext,
    root::{self, Schema},
};
use juniper::http::{GraphQLRequest, GraphQLResponse};
use lambda_http::{Body, Error, Request, Response, Service, http::Method};

async fn function_handler(
    event: Request,
    schema: &Schema,
    context: &GeneralContext,
) -> Result<Response<Body>, Error> {
    println!("Handling {} request...", event.method());
    let response = Response::builder();

    match *event.method() {
        Method::OPTIONS => response
            .status(200)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "POST")
            .header("Access-Control-Allow-Headers", "*")
            .body(Body::Empty),
        Method::POST => {
            let mut unique_context = context.new_unique_context().await;

            // get token from header
            let token = event
                .headers()
                .get("Authorization")
                .map(|v| v.to_str().unwrap().to_string());

            let user_result = match token {
                Some(token) => {
                    let user =
                        models::user::User::authenticate_from_token(&unique_context, token).await;
                    match user {
                        Ok(user) => Ok(Some(user)),
                        Err(e) => Err(GraphQLResponse::error(e)),
                    }
                }
                None => Ok(None),
            };

            let graphql_response = match user_result {
                Ok(user) => {
                    unique_context.user = user;
                    let request_string = std::str::from_utf8(event.body())?;
                    let graphql_request: GraphQLRequest = serde_json::from_str(request_string)?;
                    graphql_request.execute(schema, &unique_context).await
                }
                Err(e) => e,
            };

            let json = serde_json::to_string(&graphql_response)?;

            response
                .status(200)
                .header("content-type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "POST")
                .header("Access-Control-Allow-Headers", "*")
                .body(json.into())
        }
        _ => response
            .status(405)
            .header("Allow", "POST, OPTIONS")
            .body("405: Method not allowed - use POST instead.".into()),
    }
    .map_err(Error::from)
}

pub struct App {
    schema: Arc<Schema>,
    context: Arc<GeneralContext>,
}

impl App {
    pub async fn new() -> Result<Self, Error> {
        // There may or may not be a .env file, so we ignore the error.
        dotenv::dotenv().ok();

        Ok(Self {
            schema: Arc::new(root::create_schema()),
            context: Arc::new(graph::context::GeneralContext::new().await?),
        })
    }

    pub async fn respond(&self, event: Request) -> Result<Response<Body>, Error> {
        function_handler(event, &self.schema, &self.context).await
    }
}

impl Service<Request> for App {
    type Response = Response<Body>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, event: Request) -> Self::Future {
        let schema = self.schema.clone();
        let context = self.context.clone();

        Box::pin(async move { function_handler(event, &schema, &context).await })
    }
}
