use graph_api::App;
use lambda_http::{run, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Starting server...");

    let app = App::new().await?;

    run(app).await
}
