use diesel::prelude::*;
use juniper::{graphql_object, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A registered member")]
// In this case the Post struct is the same in the database and in the GraphQL API.
// If we needed complex resolves, we'd need to implement a #[graphql_object] manually, like for Query.
// If you need the database, remember to add context = Context.
// See the following issue for status about combining resolvers with derive:
// https://github.com/graphql-rust/juniper/issues/553
#[derive(Queryable)]
// IMPORTANT NOTE:
// The Queryable struct needs to be defined in the same way as it is in schema.rs.
// The field/column order needs to also be the same.
// See the following doc on how the types map: https://docs.rs/diesel/0.11.0/diesel/types/index.html
pub struct Post {
    pub id: i32,
    pub published: Option<bool>,
    pub title: Option<String>,
    pub body: Option<String>,
}

// pub struct NewPost<'a> {
//     pub title: Option<&'a str>,
//     pub body: Option<&'a str>,
//     pub published: Option<&'a bool>,
// }
// Again, this needs to match the Postgres schema.
#[derive(Insertable)]
#[table_name = "super::schema::posts"]
#[derive(GraphQLInputObject)]
// The GraphQL input object for creating Posts
pub struct NewPostInput {
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
}
