use graph_api::App;
use lambda_http::{run, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Starting server...");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let app = App::new().await?;

    run(app).await
}