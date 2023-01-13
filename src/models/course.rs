use crate::models::schema::courses;
use diesel::{Insertable, Queryable};
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(GraphQLObject, Serialize, Deserialize, Queryable, Insertable, Debug)]
pub struct Course {
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
}

// IMPORTANT NOTE: the struct here represents both the GraphQL object and the PostgreSQL table

// PostgreSQL:
// The Queryable struct needs to be defined in the same way as it is in schema.rs,
// and field/column order needs to also be the same.
// See the following doc on how the types map: https://docs.rs/diesel/0.11.0/diesel/types/index.html.

// GraphQL:
// If we needed complex resolvers, we'd need to impl a #[graphql_object] manually, like we did for Query.
// If you need the database, remember to add context = Context.
// See the following issue for status about combining #[graphql_object] with #[derive(GraphQLObject)]:
// https://github.com/graphql-rust/juniper/issues/553
