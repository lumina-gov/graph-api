mod graph;
mod models;

use std::sync::Arc;

use dotenv;
use graph::{
    context::Context,
    root::{self, Schema},
};
use juniper::http::GraphQLRequest;
use lambda_http::{run, service_fn, Body, Error, Request, Response};

/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(
    event: Request,
    schema: &Schema,
    context: &Context,
) -> Result<Response<Body>, Error> {
    println!("Handling {} request...", event.method());
    let response = Response::builder();

    match event.method().as_str() {
        "OPTIONS" => response
            .status(200)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "POST")
            .header("Access-Control-Allow-Headers", "Content-Type")
            .body(Body::Empty),
        "POST" => {
            let request_string = std::str::from_utf8(event.body())?;
            let graphql_request: GraphQLRequest = serde_json::from_str(request_string)?;
            let graphql_response = graphql_request.execute(schema, context).await;
            let json = serde_json::to_string(&graphql_response)?;

            response
                .status(200)
                .header("content-type", "text/json")
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
    let context = Arc::new(graph::context::create_context().await?);

    run(service_fn(|event: Request| {
        let schema = schema.clone();
        let context = context.clone();

        async move { function_handler(event, &schema, &context).await }
    }))
    .await
}
