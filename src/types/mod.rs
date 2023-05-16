// PostgreSQL:
// The Queryable struct needs to be defined in the same way as it is in schema.rs,
// and field/column order needs to also be the same.
// See the following doc on how the types map: https://docs.rs/diesel/0.11.0/diesel/types/index.html.

// GraphQL:
// If we needed complex resolvers, we'd need to impl a #[graphql_object] manually, like we did for Query.
// If you need the database, remember to add context = Context.
// See the following issue for status about combining #[graphql_object] with #[derive(GraphQLObject)]:
// https://github.com/graphql-rust/juniper/issues/553

pub mod citizenship_application;
pub mod applications;
pub mod organisation;
pub mod unit_progress;
pub mod user;
pub mod utils;
pub mod question_assessment;