
mod graph;
mod models;

use std::sync::Arc;

use graph::{resolvers::{self, Schema}, context::Context};
use juniper::http::GraphQLRequest;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use dotenv;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request, schema: &Schema, context: &Context) -> Result<Response<Body>, Error> {
    if event.method() == "OPTIONS" {
        let resp = Response::builder()
            .status(200)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "POST")
            .header("Access-Control-Allow-Headers", "Content-Type")
            .body(Body::Empty)
            .map_err(Box::new)?;
        Ok(resp)
    } else if event.method() == "POST" {
        let request_string = std::str::from_utf8(event.body()).unwrap();
        let graphql_request: GraphQLRequest = serde_json::from_str(request_string).unwrap();

        let res = graphql_request.execute(schema, context).await;
        let data = serde_json::to_string(&res).unwrap();

        // Return something that implements IntoResponse.
        // It will be serialized to the right response event automatically by the runtime
        let resp = Response::builder()
            .status(200)
            .header("content-type", "text/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(data.into())
            .map_err(Box::new)?;
        Ok(resp)
    } else {
        let resp = Response::builder()
            .status(405)
            .body("Method not allowed".into())
            .map_err(Box::new)?;
        Ok(resp)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let schema = Arc::new(resolvers::create_schema());
    let context = Arc::new(graph::context::create_context().await?);

    run(service_fn(|event: Request| {
        let schema = schema.clone();
        let context = context.clone();

        async move {
            function_handler(event, &schema, &context).await
        }
    })).await
}
