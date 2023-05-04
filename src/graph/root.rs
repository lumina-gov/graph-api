use crate::error::ErrorCode;
use crate::models::citizenship_application::CitizenshipApplication;
use crate::models::citizenship_application::CitizenshipApplicationInput;
use crate::models::question_assessment::QuestionAssessment;
use crate::models::unit_progress::UnitProgress;
use crate::models::unit_progress::UnitStatus;
use crate::models::user::CreateUserInput;
use crate::models::user::LoginUserInput;
use crate::models::user::User;
use crate::stripe::get_stripe_client;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use juniper::IntoFieldError;
use juniper::{graphql_object, EmptySubscription, FieldResult};
use std::str::FromStr;
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
    async fn ping(&self) -> FieldResult<String> {
        Ok("pong".to_string())
    }

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

    // Get the number of users grouped by their creation date, for a specified interval
    // in months, and the count of intervals to go back in time.
    // For example, if interval is 1 and count is 12, this will return the number of users
    // created in the last 12 months, grouped by month.
    async fn user_count_by_interval(
        &self,
        context: &UniqueContext,
        interval: i32,
        count: i32,
    ) -> FieldResult<Vec<i32>> {
        if count > 36 {
            return Err(
                ErrorCode::Custom("BAD_REQUEST".into(), "Count must be below 36".into())
                    .into_field_error(),
            );
        }
        if interval > 6 {
            return Err(
                ErrorCode::Custom("BAD_REQUEST".into(), "Interval must be below 6".into())
                    .into_field_error(),
            );
        }

        use crate::models::schema::users::dsl::*;

        let mut conn = context.diesel_pool.get().await?;

        // create a duration (interval * month)
        let duration = chrono::Duration::days(interval as i64 * 30);

        // get the current time
        let now = chrono::Utc::now();

        // for count times, get the number of users created in the last interval
        let mut data = Vec::new();

        for i in 0..count {
            let start = now - duration * i;
            let end = now - duration * (i + 1);

            let count: i64 = users
                .filter(joined.gt(end))
                .filter(joined.lt(start))
                .count()
                .get_result(&mut conn)
                .await?;

            data.push(count as i32);
        }

        Ok(data)
    }

    async fn user_count(context: &UniqueContext) -> FieldResult<i32> {
        use crate::models::schema::users::dsl::*;

        let data: i64 = users
            .count()
            .get_result(&mut context.diesel_pool.get().await?)
            .await?;

        Ok(data as i32)
    }

    async fn me(context: &UniqueContext) -> Option<User> {
        context.user().ok()
    }

    async fn course_progress(
        context: &UniqueContext,
        course_slug: String,
    ) -> FieldResult<Vec<UnitProgress>> {
        let user = match &context.user {
            None => return Err(ErrorCode::Unauthenticated.into_field_error()),
            Some(user) => user,
        };

        let progress = UnitProgress::course_progres(context, user, course_slug).await?;

        Ok(progress)
    }

    async fn all_course_progress(context: &UniqueContext) -> FieldResult<Vec<Vec<UnitProgress>>> {
        let user = match &context.user {
            None => return Err(ErrorCode::Unauthenticated.into_field_error()),
            Some(user) => user,
        };

        Ok(UnitProgress::all_course_progress(context, user).await?)
    }

    async fn last_updated_unit(context: &UniqueContext) -> FieldResult<Option<UnitProgress>> {
        let user = match &context.user {
            None => return Err(ErrorCode::Unauthenticated.into_field_error()),
            Some(user) => user,
        };

        Ok(UnitProgress::last_updated_unit(context, user).await?)
    }

    async fn question_assessment(
        context: &UniqueContext,
        course_slug: String,
        unit_slug: String,
        question_slug: String,
    ) -> FieldResult<Option<QuestionAssessment>> {
        let user = match &context.user {
            None => return Ok(None),
            Some(user) => user,
        };

        Ok(QuestionAssessment::get_question_assessment(
            context,
            user,
            course_slug,
            unit_slug,
            question_slug,
        ).await?)
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
        create_session.line_items = Some(vec![stripe::CreateCheckoutSessionLineItems {
            price: Some(String::from(&dotenv::var("LIGHT_UNIVERSITY_PRICE_ID")?)),
            quantity: Some(1),
            ..Default::default()
        }]);

        let session = stripe::CheckoutSession::create(&client, create_session).await?;
        match session.url {
            None => return Err(ErrorCode::CouldNotCreateCheckoutSession.into_field_error()),
            Some(url) => Ok(url),
        }
    }

    async fn set_unit_progress(
        &self,
        context: &UniqueContext,
        unit_slug: String,
        course_slug: String,
        status: UnitStatus,
    ) -> FieldResult<UnitProgress> {
        let user = match &context.user {
            None => return Err(ErrorCode::Unauthenticated.into_field_error()),
            Some(user) => user,
        };

        let unit_progress =
            UnitProgress::create_or_update(context, &user, unit_slug, course_slug, status).await?;

        Ok(unit_progress)
    }

    async fn question_assessment(
        &self,
        context: &UniqueContext,
        course_slug: String,
        unit_slug: String,
        question_slug: String,
        question: String,
        answer: String,
    ) -> FieldResult<QuestionAssessment> {
        let user = match &context.user {
            None => return Err(ErrorCode::Unauthenticated.into_field_error()),
            Some(user) => user,
        };

        Ok(QuestionAssessment::create_assessment(
            context,
            user,
            course_slug,
            unit_slug,
            question_slug,
            question,
            answer,
        ).await?)
    }
}
