use std::str::FromStr;

use crate::error::ErrorCode;
use crate::models::citizenship_application::CitizenshipApplication;
use crate::models::citizenship_application::CitizenshipApplicationInput;
use crate::models::course::Course;
use crate::models::enrollments::Enrollment;
use crate::models::schema::courses;
use crate::models::unit::Unit;
use crate::models::user::CreateUserInput;
use crate::models::user::LoginUserInput;
use crate::models::user::User;
use crate::stripe::get_stripe_client;
use diesel::QueryDsl;
use diesel::OptionalExtension;
use diesel_async::RunQueryDsl;
use juniper::IntoFieldError;
use juniper::{graphql_object, EmptySubscription, FieldResult};
use uuid::Uuid;
use zxcvbn::time_estimates::CrackTimeSeconds;

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
    rename_all = "none"
)]
impl Query {
    /// Returns the crack time of a password
    /// Used for password strength estimation
    /// On the frontend
    async fn crack_time(&self, password: String) -> FieldResult<CrackSeconds> {
        let guesses = match zxcvbn::zxcvbn(&password, &[]) {
            Ok(entropy) => entropy.guesses(),
            Err(_) => 0,
        } as f64;

        Ok(CrackSeconds {
            guesses,
            seconds: guesses as f64 / 100_000.0,
            string: CrackTimeSeconds::Float(guesses as f64 / 100_000.0).to_string(),
        })
    }

    async fn user_count(context: &UniqueContext) -> FieldResult<i32> {
        use crate::models::schema::users::dsl::*;

        let data: i64 = users
            .count()
            .get_result(&mut context.diesel_pool.get().await?)
            .await?;

        Ok(data as i32)
    }

    async fn courses(context: &UniqueContext) -> FieldResult<Vec<Course>> {
        use crate::models::schema::courses::dsl::*;

        let data = courses
            .load::<Course>(&mut context.diesel_pool.get().await?)
            .await?;

        Ok(data)
    }
    async fn course_by_slug(context: &UniqueContext, slug: String) -> FieldResult<Option<Course>> {
        use crate::models::schema::courses::dsl;
        let conn = &mut context.diesel_pool.get().await?;

        let data = dsl::courses
            .filter(dsl::slug.eq(slug))
            .first::<Course>(conn)
            .await
            .optional()?;

        Ok(data)
    }

    async fn unit_by_slug(context: &UniqueContext, slug: String) -> FieldResult<Option<Unit>> {
        use crate::models::schema::units::dsl;
        let conn = &mut context.diesel_pool.get().await?;

        let data = dsl::units
            .filter(dsl::slug.eq(slug))
            .first::<Unit>(conn)
            .await
            .optional()?;

        Ok(data)
    }

    async fn me(context: &UniqueContext) -> Option<User> {
        context.user().ok()
    }
}

#[graphql_object(
    context = UniqueContext
    rename_all = "none"
)]
impl Mutation {
    fn test() -> String {
        "Hello World!".into()
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

    async fn create_citizenship_application(
        &self,
        context: &UniqueContext,
        citizenship_application: CitizenshipApplicationInput,
    ) -> FieldResult<Uuid> {
        CitizenshipApplication::create_citizenship_application(context, citizenship_application)
            .await
    }

    async fn create_light_university_checkout_session(
        &self,
        context: &UniqueContext,
        success_url: String,
    ) -> FieldResult<String> {
        let user = &context.user;
        let stripe_customer_id = match user {
            None => return Err(ErrorCode::Unauthenticated.into_field_error()),
            Some(user) => user.stripe_customer_id(context).await?,
        };

        let client = get_stripe_client();
        let mut create_session = stripe::CreateCheckoutSession::new(&success_url);
        create_session.customer = Some(stripe::CustomerId::from_str(&stripe_customer_id)?);
        create_session.mode = Some(stripe::CheckoutSessionMode::Subscription);
        create_session.line_items = Some(vec![
            stripe::CreateCheckoutSessionLineItems {
                price: Some(String::from(dotenv::var("LIGHT_UNIVERSITY_PRICE_ID").expect("LIGHT_UNIVERSITY_PRICE_ID not set"))),
                quantity: Some(1),
                ..Default::default()
            },
        ]);

        let session = stripe::CheckoutSession::create(&client, create_session).await?;
        match session.url {
            None => return Err(ErrorCode::CouldNotCreateCheckoutSession.into_field_error()),
            Some(url) => Ok(url),
        }
    }

    async fn enroll_user_to_course(
        &self,
        context: &UniqueContext,
        course_slug: String,
    ) -> FieldResult<bool> {
        let user = match &context.user {
            None => return Err(ErrorCode::Unauthenticated.into_field_error()),
            Some(user) => user,
        };

        let course = match courses::table
            .filter(courses::slug.eq(course_slug))
            .first::<Course>(&mut context.diesel_pool.get().await?)
            .await
            .optional()? {
            None => return Err(ErrorCode::CourseNotFound.into_field_error()),
            Some(course) => course,
        };

        Enrollment::enroll_user(context, user, &course).await?;

        Ok(true)
    }
}
