use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;

pub struct Context {
    // Use your real database pool here.
    pub diesel_pool: Pool<AsyncPgConnection>,
}
// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

pub async fn create_context() -> Result<Context, anyhow::Error> {
    let postgrest_url: String = dotenv::var("DATABASE_URL")
        .expect("DATABASE_URL not set in .env");

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(postgrest_url);
    let pool = Pool::builder(config).build()?;

    Ok(Context {
        diesel_pool: pool,
    })
}