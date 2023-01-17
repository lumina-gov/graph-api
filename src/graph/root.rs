use crate::models::citizenship_application::CitizenshipApplication;
use crate::models::citizenship_application::CitizenshipApplicationInput;
use crate::models::course::Course;
use crate::models::course::CourseInsertable;
use crate::models::course::CreateCourseInput;
use crate::models::flexible_application::ApplicationType;
use crate::models::schema::courses::dsl::courses;
use crate::models::schema::users::dsl::users;
use crate::models::user::CreateUserInput;
use crate::models::user::LoginUserInput;
use crate::models::user::User;
use diesel::associations::HasTable;
use diesel::insert_into;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use juniper::{graphql_object, EmptySubscription, FieldResult};
use uuid::Uuid;
use zxcvbn::time_estimates::CrackTimeSeconds;

use super::context::GeneralContext;
use super::context::UniqueContext;
use super::misc::CrackSeconds;
use diesel::ExpressionMethods;

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
    /// Returns the crack time of a password
    /// Used for password strength estimation
    /// On the frontend
    async fn crack_time(&self, password: String) -> FieldResult<CrackSeconds> {
        let guesses = match zxcvbn::zxcvbn(&password, &[]) {
            Ok(entropy) => entropy.guesses(),
            Err(_) => 0,
        } as i32;

        Ok(CrackSeconds {
            guesses,
            seconds: guesses as f64 / 100_000.0,
            string: CrackTimeSeconds::Float(guesses as f64 / 100_000.0).to_string(),
        })
    }

    async fn user_count(context: &UniqueContext) -> FieldResult<i32> {
        let data: i64 = users
            .count()
            .get_result(&mut context.diesel_pool.get().await?)
            .await?;

        Ok(data as i32)
    }

    async fn create_citizenship_application(
        &self,
        context: &UniqueContext,
        citizenship_application: CitizenshipApplicationInput,
    ) -> FieldResult<CitizenshipApplication> {
        CitizenshipApplication::create_citizenship_application(context, citizenship_application)
            .await
    }

    async fn courses(context: &UniqueContext) -> FieldResult<Vec<Course>> {
        let data = courses
            .load::<Course>(&mut context.diesel_pool.get().await?)
            .await?;

        Ok(data)
    }
    async fn course_by_slug(context: &UniqueContext, slug: String) -> FieldResult<Course> {
        use crate::models::schema::courses::*;
        let conn = &mut context.diesel_pool.get().await;

        let data = courses
            .filter(dsl::slug.eq(slug))
            .first::<Course>(conn)
            .await?;

        Ok(data)
    }

    async fn me(context: &UniqueContext) -> FieldResult<User> {
        context.user()
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

    /// Returns a JWT token for the user
    async fn login(context: &UniqueContext, login_user: LoginUserInput) -> FieldResult<String> {
        User::login_user(context, login_user).await
    }
    fn submit_application(
        context: &UniqueContext,
        bson: String,
        application_type: ApplicationType,
    ) -> FieldResult<bool> {
        // TODO:
        // 1. Check that user has no applications that are of application_type, else Error()
        // 2. Insert user_id, application_type, bson and status = received into the flexible_applications table
        Ok(true)
    }
}
