mod error;
mod graph;
mod models;

use std::sync::Arc;

use dotenv;
use graph::{
    context::GeneralContext,
    root::{self, Schema},
};
use juniper::http::{GraphQLRequest, GraphQLResponse};
use lambda_http::{run, service_fn, Body, Error, Request, Response};

/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(
    event: Request,
    schema: &Schema,
    context: &GeneralContext,
) -> Result<Response<Body>, Error> {
    println!("Handling {} request...", event.method());
    let response = Response::builder();

    let mut unique_context = context.new_unique_context().await;

    match event.method().as_str() {
        "OPTIONS" => response
            .status(200)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "POST")
            .header("Access-Control-Allow-Headers", "Content-Type")
            .body(Body::Empty),
        "POST" => {
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
                .body(json.into())
        }
        _ => response
            .status(405)
            .header("Allow", "POST, OPTIONS")
            .body("405: Method not allowed - use POST instead.".into()),
    }
    .map_err(Error::from)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Starting server...");
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let schema = Arc::new(root::create_schema());
    let context = Arc::new(graph::context::GeneralContext::new().await?);

    run(service_fn(|event: Request| {
        let schema = schema.clone();
        let context = context.clone();

        async move { function_handler(event, &schema, &context).await }
    }))
    .await
}
