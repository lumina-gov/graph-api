use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Serialize, Deserialize};

use super::utils::model::Model;

#[derive(GraphQLObject, Serialize, Deserialize)]
#[graphql(description = "A registered member")]
// In this case the Post struct is the same in the database and in the GraphQL API.
// If we needed complex resolves, we'd need to implement a #[graphql_object] manually, like for Query.
// If you need the database, remember to add context = Context.
// See the following issue for status about combining resolvers with derive:
// https://github.com/graphql-rust/juniper/issues/553
pub struct Post {
    pub id: i32,
    pub published: bool,
    pub title: String,
    pub body: String,
}

impl Model for Post {
    fn collection_name() -> String {
        "posts".to_string()
    }
}
// pub struct NewPost<'a> {
//     pub title: Option<&'a str>,
//     pub body: Option<&'a str>,
//     pub published: Option<&'a bool>,
// }
// Again, this needs to match the Postgres schema.
#[derive(GraphQLInputObject)]
// The GraphQL input object for creating Posts
pub struct NewPostInput {
    pub title: String,
    pub body: String,
    pub published: bool,
}
