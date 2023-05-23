use std::str::FromStr;

use crate::{
    applications::{CitizenshipApplication, CitizenshipStatus},
    error::APIError,
    schema::users,
    util::stripe::get_stripe_client,
};
use async_graphql::{ComplexObject, Context, Enum, SimpleObject};
use chrono::{DateTime, NaiveDateTime, Utc};
use sea_orm::{
    sea_query::{extension::sqlite::SqliteExpr, Expr},
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use stripe::{CreateBillingPortalSession, PriceId};

pub type User = users::Model;
pub type UserActiveModel = users::ActiveModel;
pub type UserColumn = users::Column;

#[derive(Debug, Clone, SimpleObject, Deserialize, Serialize)]
#[graphql(rename_fields = "snake_case", rename_args = "snake_case")]
struct SubscriptionInfo {
    status: SubscriptionStatus,
    expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Copy, Eq, PartialEq, Clone, Deserialize, Serialize, Enum)]
enum SubscriptionStatus {
    Renewing,
    Expiring,
    None,
}

#[ComplexObject(rename_fields = "snake_case", rename_args = "snake_case")]
impl User {
    async fn roles(&self) -> Vec<String> {
        //TODO frontend wants an array of roles, so implement as an array
        match &self.role {
            Some(role) => vec![role.clone()],
            None => vec![],
        }
    }

    async fn referral_count(&self, ctx: &Context<'_>) -> Result<u64, anyhow::Error> {
        let conn = ctx.data_unchecked::<DatabaseConnection>();

        let count = users::Entity::find()
            .filter(users::Column::Referrer.eq(self.id))
            .count(conn)
            .await?;

        Ok(count)
    }

    async fn citizenship_status(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<CitizenshipStatus>, anyhow::Error> {
        use crate::schema::applications;

        let conn = ctx.data_unchecked::<DatabaseConnection>();

        match applications::Entity::find()
            .filter(applications::Column::ApplicationType.eq("citizenship"))
            .filter(Expr::col(applications::Column::Application).cast_json_field("user_id"))
            .order_by_desc(applications::Column::CreatedAt)
            .one(conn)
            .await?
        {
            Some(application) => {
                let json: CitizenshipApplication = serde_json::from_value(application.application)?;
                Ok(Some(json.citizenship_status))
            }
            None => Ok(None),
        }
    }

    async fn customer_portal_url(
        &self,
        ctx: &Context<'_>,
        return_url: Option<String>,
    ) -> Result<String, anyhow::Error> {
        let stripe_customer_id = self.stripe_customer_id(ctx).await?;
        let client = get_stripe_client();

        let mut session =
            CreateBillingPortalSession::new(stripe::CustomerId::from_str(&stripe_customer_id)?);
        session.return_url = return_url.as_deref();

        let session = stripe::BillingPortalSession::create(&client, session).await?;

        Ok(session.url)
    }

    async fn stripe_subscription_info(
        &self,
        ctx: &Context<'_>,
    ) -> Result<SubscriptionInfo, anyhow::Error> {
        let stripe_customer_id = self.stripe_customer_id(ctx).await?;
        let client = get_stripe_client();

        let subscription = stripe::Subscription::list(
            &client,
            &stripe::ListSubscriptions {
                customer: Some(stripe::CustomerId::from_str(&stripe_customer_id)?),
                price: Some(PriceId::from_str(&dotenv::var(
                    "LIGHT_UNIVERSITY_PRICE_ID",
                )?)?),
                ..Default::default()
            },
        )
        .await?
        .data
        .get(0)
        .cloned();

        match subscription {
            Some(subscription) => {
                let native_date_time =
                    match NaiveDateTime::from_timestamp_millis(subscription.current_period_end) {
                        Some(date) => date,
                        None => {
                            return Err(APIError::new(
                                "FAILED_TO_PARSE_DATE",
                                "Failed to parse date",
                            )
                            .into())
                        }
                    };

                let subscription_expiry_date = DateTime::<Utc>::from_utc(native_date_time, Utc);

                Ok(SubscriptionInfo {
                    expiry_date: Some(subscription_expiry_date),
                    status: match subscription.cancel_at_period_end {
                        true => SubscriptionStatus::Expiring,
                        false => SubscriptionStatus::Renewing,
                    },
                })
            }
            None => Ok(SubscriptionInfo {
                status: SubscriptionStatus::None,
                expiry_date: None,
            }),
        }
    }
    pub async fn stripe_customer_id(&self, ctx: &Context<'_>) -> Result<String, anyhow::Error> {
        let conn = ctx.data_unchecked::<DatabaseConnection>();

        let client = get_stripe_client();

        // use stripe_customer_id if it exists
        match &self.stripe_customer_id {
            Some(customer) => Ok(customer.clone()),
            None => {
                let customer = stripe::Customer::create(
                    &client,
                    stripe::CreateCustomer {
                        name: Some(&format!("{} {}", self.first_name, self.last_name)),
                        email: Some(&self.email),
                        metadata: Some(
                            [("user_id".into(), self.id.to_string())]
                                .into_iter()
                                .collect(),
                        ),
                        ..Default::default()
                    },
                )
                .await?;

                // update user with stripe_customer_id
                let user = users::ActiveModel {
                    stripe_customer_id: Set(Some(customer.id.to_string())),
                    ..Default::default()
                };

                user.update(conn).await?;

                Ok(customer.id.to_string())
            }
        }
    }
}
