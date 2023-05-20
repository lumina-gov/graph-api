use std::str::FromStr;

use anyhow::Result;
use async_graphql::{Context, MergedObject, Object};
use uuid::Uuid;

use crate::{
    error::APIError,
    graphql::{
        citizenship_application::{CitizenshipApplication, CitizenshipApplicationInput},
        user::{User, UserMutation},
    },
    guards::auth::AuthGuard,
    schema::{
        question_assessments::QuestionAssessment, sea_orm_active_enums::UnitStatus,
        unit_progress::UnitProgress,
    },
    stripe::get_stripe_client,
};

#[derive(MergedObject, Default)]
pub(crate) struct Mutation(BaseMutation, UserMutation);

#[derive(Default)]
struct BaseMutation;

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl BaseMutation {
    #[graphql(guard = "AuthGuard")]
    async fn create_citizenship_application(
        &self,
        ctx: &Context<'_>,
        citizenship_application: CitizenshipApplicationInput,
    ) -> Result<Uuid> {
        CitizenshipApplication::create_citizenship_application(ctx, citizenship_application).await
    }

    #[graphql(guard = "AuthGuard")]
    async fn create_light_university_checkout_session(
        &self,
        ctx: &Context<'_>,
        success_url: String,
    ) -> Result<String, anyhow::Error> {
        let user = ctx.data_unchecked::<User>();
        let stripe_customer_id = user.stripe_customer_id().await?;

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
            None => Err(APIError::new(
                "COULD_NOT_CREATE_CHECKOUT_SESSION",
                "Could not create checkout session",
            )
            .into()),
            Some(url) => Ok(url),
        }
    }

    #[graphql(guard = "AuthGuard")]
    async fn set_unit_progress(
        &self,
        ctx: &Context<'_>,
        unit_slug: String,
        course_slug: String,
        status: UnitStatus,
    ) -> Result<UnitProgress, anyhow::Error> {
        let user = ctx.data_unchecked::<User>();

        let unit_progress =
            UnitProgress::create_or_update(ctx, user, unit_slug, course_slug, status).await?;

        Ok(unit_progress)
    }

    #[graphql(guard = "AuthGuard")]
    async fn question_assessment(
        &self,
        ctx: &Context<'_>,
        course_slug: String,
        unit_slug: String,
        question_slug: String,
        question: String,
        answer: String,
        question_context: Option<String>,
    ) -> Result<QuestionAssessment, anyhow::Error> {
        let user = ctx.data_unchecked::<User>();

        QuestionAssessment::create_assessment(
            ctx,
            user,
            course_slug,
            unit_slug,
            question_slug,
            question,
            answer,
            question_context,
        )
        .await
    }
}
