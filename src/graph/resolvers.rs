use crate::models::{course::Course};
use crate::models::schema::courses::dsl::*;
use super::context::Context;
use chrono::DateTime;
use diesel::query_dsl::methods::FilterDsl;
use diesel_async::RunQueryDsl;
use juniper::{graphql_object, EmptySubscription, FieldResult};
use uuid::Uuid;

pub struct Query;
pub struct Mutation;

// A root schema consists of a query, a mutation, and a subscription.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

#[graphql_object(
    // Here we specify the context type for the object.
    // We need to do this in every type that
    // needs access to the context.
    context = Context
)]

impl Query {
    async fn courses(context: &Context) -> FieldResult<Vec<Course>> {
        let data = courses
            .load::<Course>(&mut context.diesel_pool.get().await?)
            .await?;

        Ok(data)
    }
}

#[graphql_object(
    context = Context
)]
impl Mutation {
    fn test() -> String {
        "Hello World!".into()
    }
}