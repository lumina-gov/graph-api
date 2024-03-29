use std::str::FromStr;

use crate::error::new_err;
use crate::guards::auth::AuthGuard;
use crate::util::variables::SECRET_VARIABLES;
use crate::{graphql::types::user::User, util::stripe::get_stripe_client};
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
    ) -> async_graphql::Result<String> {
        let user = ctx.data_unchecked::<User>();
        let stripe_customer_id = user.stripe_customer_id(ctx).await?;

        let client = get_stripe_client();
        let mut create_session = stripe::CreateCheckoutSession::new(&success_url);
        create_session.customer = Some(stripe::CustomerId::from_str(&stripe_customer_id)?);
        create_session.mode = Some(stripe::CheckoutSessionMode::Subscription);
        create_session.line_items = Some(vec![stripe::CreateCheckoutSessionLineItems {
            price: Some(String::from(&SECRET_VARIABLES.light_university_product_id)),
            quantity: Some(1),
            ..Default::default()
        }]);

        let session = stripe::CheckoutSession::create(&client, create_session).await?;
        match session.url {
            None => Err(new_err(
                "COULD_NOT_CREATE_CHECKOUT_SESSION",
                "Could not create checkout session",
            )),
            Some(url) => Ok(url),
        }
    }
}
