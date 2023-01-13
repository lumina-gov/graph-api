use super::context::Context;
use super::models::{NewPostInput, Post};
use diesel::{insert_into, RunQueryDsl};
use diesel::{PgConnection, QueryDsl};
use juniper::{graphql_object, EmptySubscription, FieldResult};

pub struct Query;
#[graphql_object(
    // Here we specify the context type for the object.
    // We need to do this in every type that
    // needs access to the context.
    context = Context
)]
impl Query {
    fn apiVersion() -> &'static str {
        "1.0"
    }

    // #[graphql(name = "postById")] is redundant
    fn post_by_id(context: &Context, post_id: i32) -> FieldResult<Post> {
        use super::schema::posts::dsl::*;
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        let post = posts
            .find(post_id)
            .first::<Post>(conn)
            .expect("Error connecting!");

        Ok(post)
    }
}

pub struct Mutation;
#[graphql_object(
    context = Context
)]
impl Mutation {
    fn apiVersion() -> &'static str {
        "1.0"
    }
    pub fn new_post(context: &Context, input: NewPostInput) -> FieldResult<Post> {
        use super::schema::posts::dsl::*;
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();
        let res = insert_into(posts)
            .values(&input)
            .get_result::<Post>(conn)
            .unwrap();
        Ok(res)
    }
}

// A root schema consists of a query, a mutation, and a subscription.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
