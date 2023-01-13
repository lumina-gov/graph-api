use lambda_http::{run, service_fn, Body, Error, Request, Response};
mod graph;
use dotenv;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
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
        let query_string = std::str::from_utf8(event.body()).expect("Invalid utf-8 sequence");

        let data = graph::endpoint::run_request(query_string);

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

    run(service_fn(function_handler)).await
}
