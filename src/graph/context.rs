use std::sync::Arc;

pub struct Context {
    // Use your real database pool here.
    pub postgrest_client: Arc<postgrest::Postgrest>,
    pub postgrest_jwt: String
}
// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

pub fn create_context() -> Context {
    let postgrest_url: String = dotenv::var("POSTGREST_URL")
        .expect("POSTGREST_URL not set in .env");

    let postgrest_jwt: String = dotenv::var("POSTGREST_JWT")
        .expect("POSTGREST_JWT not set in .env");

    Context {
        postgrest_client: Arc::new(postgrest::Postgrest::new(postgrest_url)),
        postgrest_jwt
    }
}