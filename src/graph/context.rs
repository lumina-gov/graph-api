use super::postgres::PostgresPool;

pub struct Context {
    // Use your real database pool here.
    pub pool: PostgresPool,
}
// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

pub fn create_context() -> Context {
    Context {
        pool: super::postgres::establish_pool(),
    }
}
