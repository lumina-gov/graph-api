use super::context::Context;
use crate::models::course::Course;
use crate::models::course::CourseInsertable;
use crate::models::course::CreateCourseInput;
use crate::models::schema::courses::dsl::*;
use diesel::insert_into;
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
    async fn create_course(context: &Context, course: CreateCourseInput) -> FieldResult<Course> {
        let conn = &mut context.diesel_pool.get().await?;
        Ok(insert_into(courses)
            .values(CourseInsertable {
                id: Uuid::new_v4(),
                name: course.name,
            })
            .get_result(conn)
            .await?)
    }
}
