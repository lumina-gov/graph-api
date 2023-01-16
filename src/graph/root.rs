use crate::error::ErrorCode;
use crate::models::course::Course;
use crate::models::course::CourseInsertable;
use crate::models::course::CreateCourseInput;
use crate::models::schema::courses::dsl::*;
use crate::models::user::CreateUserInput;
use crate::models::user::User;
use diesel::insert_into;
use diesel_async::RunQueryDsl;
use juniper::IntoFieldError;
use juniper::{graphql_object, EmptySubscription, FieldResult};
use uuid::Uuid;

use super::context::UniqueContext;

pub struct Query;
pub struct Mutation;

// A root schema consists of a query, a mutation, and a subscription.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<UniqueContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

#[graphql_object(
    // Here we specify the context type for the object.
    // We need to do this in every type that
    // needs access to the context.
    context = UniqueContext
)]

impl Query {
    async fn courses(context: &UniqueContext) -> FieldResult<Vec<Course>> {
        let data = courses
            .load::<Course>(&mut context.diesel_pool.get().await?)
            .await?;

        Ok(data)
    }
    async fn me(context: &UniqueContext) -> FieldResult<User> {
        match context.user.clone() {
            Some(user) => Ok(user),
            None => Err(ErrorCode::Unauthenticated.into()),
        }
    }
}

#[graphql_object(
    context = UniqueContext
)]
impl Mutation {
    fn test() -> String {
        "Hello World!".into()
    }
    async fn create_course(
        context: &UniqueContext,
        course: CreateCourseInput,
    ) -> FieldResult<Course> {
        let conn = &mut context.diesel_pool.get().await?;
        Ok(insert_into(courses)
            .values(CourseInsertable {
                id: Uuid::new_v4(),
                name: course.name,
            })
            .get_result(conn)
            .await?)
    }

    async fn create_user(
        context: &UniqueContext,
        create_user_input: CreateUserInput,
    ) -> FieldResult<Uuid> {
        User::create_user(context, create_user_input).await
    }
}
