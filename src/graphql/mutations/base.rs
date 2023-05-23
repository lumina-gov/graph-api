use std::str::FromStr;

use crate::guards::auth::AuthGuard;
use crate::{error::APIError, graphql::types::user::User, util::stripe::get_stripe_client};
use anyhow::Result;
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct BaseMutation;

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl BaseMutation {
    #[graphql(guard = "AuthGuard")]
    async fn create_light_university_checkout_session(
        &self,
        ctx: &Context<'_>,
        success_url: String,
    ) -> Result<String, anyhow::Error> {
        let user = ctx.data_unchecked::<User>();
        let stripe_customer_id = user.stripe_customer_id(ctx).await?;

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
}
