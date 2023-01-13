use super::context::Context;
use crate::models::{post::{NewPostInput, Post}, utils::model::Model};
use juniper::{graphql_object, EmptySubscription, FieldResult};

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
    fn apiVersion() -> &'static str {
        "1.0"
    }

    // #[graphql(name = "postById")] is redundant
    async fn post_by_id(context: &Context, post_id: i32) -> FieldResult<Post> {
        let post = context.postgrest_client
            .from("posts")
            .auth(context.postgrest_jwt.as_str())
            .select("*")
            .eq("id", post_id.to_string())
            .single()
            .execute()
            .await?;

        panic!("{:#?}", post);
    }
}

#[graphql_object(
    context = Context
)]
impl Mutation {
    fn apiVersion() -> &'static str {
        "1.0"
    }
    pub fn new_post(_context: &Context, input: NewPostInput) -> FieldResult<Post> {
        Ok(Post {
            id: 1,
            published: true,
            title: input.title,
            body: input.body,
        })
    }
}