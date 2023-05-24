use graph_api::App;
use lambda_http::{run, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Starting server...");
    // There is not a .env file in prod, so we ignore the error.
    dotenv::dotenv().ok();
    let open_ai_key = dotenv::var("OPENAI_KEY").expect("OPENAI_KEY not set in .env");
    let postgress_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL not set in .env");
    let app = App::new(&postgress_url, &open_ai_key).await?;

    run(app).await
}
