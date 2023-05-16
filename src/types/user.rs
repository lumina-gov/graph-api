use std::str::FromStr;
use std::time::SystemTime;

use super::applications::Application;
use super::citizenship_application::{CitizenshipApplication, CitizenshipStatus};
use super::utils::jsonb::JsonB;
use crate::db_schema::{applications, users};
use crate::error::APIError;
use crate::guards::auth::AuthGuard;
use crate::stripe::get_stripe_client;
use crate::DieselPool;
use async_graphql::{ComplexObject, Context, Enum, Object, SimpleObject};
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{
    result::DatabaseErrorKind, Identifiable, Insertable, OptionalExtension, QueryDsl, Queryable,
};
use diesel::{ExpressionMethods, PgJsonbExpressionMethods};
use diesel_async::RunQueryDsl;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use stripe::{CreateBillingPortalSession, PriceId};
use uuid::Uuid;

#[derive(Default)]
pub struct UserQuery;

#[derive(Default)]
pub struct UserMutation;

#[derive(
    Debug, Clone, Deserialize, Serialize, Identifiable, Queryable, Insertable, SimpleObject,
)]
#[graphql(complex)]
#[graphql(rename_fields = "snake_case", rename_args = "snake_case")]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(with = "ts_milliseconds")]
    pub joined: DateTime<Utc>,
    #[graphql(skip)]
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub calling_code: String,
    pub country_code: String,
    pub phone_number: String,
    pub role: Option<String>,
    #[graphql(skip)]
    pub referrer: Option<Uuid>,
    #[graphql(skip)]
    pub stripe_customer_id: Option<String>,
}

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

#[derive(Debug, Serialize, Deserialize)]
struct TokenData {
    user_id: Uuid,
    created: SystemTime,
}

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl UserQuery {
    async fn user_count(&self, ctx: &Context<'_>) -> Result<i32, anyhow::Error> {
        use crate::db_schema::users::dsl::*;
        let mut conn = &mut ctx.data_unchecked::<DieselPool>().get().await?;
        let data: i64 = users.count().get_result(&mut conn).await?;

        Ok(data as i32)
    }

    // Get the number of users grouped by their creation date, for a specified interval
    // in months, and the count of intervals to go back in time.
    // For example, if interval is 1 and count is 12, this will return the number of users
    // created in the last 12 months, grouped by month.
    async fn user_count_by_interval(
        &self,
        ctx: &Context<'_>,
        interval: i32,
        count: i32,
    ) -> Result<Vec<i32>, anyhow::Error> {
        if count > 36 {
            return Err(APIError::new("BAD_REQUEST", "Count must be below 36").into());
        }
        if interval > 6 {
            return Err(APIError::new("BAD_REQUEST", "Interval must be below 6").into());
        }

        let mut conn = &mut ctx.data_unchecked::<DieselPool>().get().await?;

        // create a duration (interval * month)
        let duration = chrono::Duration::days(interval as i64 * 30);

        // get the current time
        let now = chrono::Utc::now();

        // for count times, get the number of users created in the last interval
        let mut data = Vec::new();

        for i in 0..count {
            let start = now - duration * i;
            let end = now - duration * (i + 1);

            let count: i64 = users::table
                .filter(users::joined.gt(end))
                .filter(users::joined.lt(start))
                .count()
                .get_result(&mut conn)
                .await?;

            data.push(count as i32);
        }

        Ok(data)
    }

    #[graphql(guard = "AuthGuard")]
    async fn me(&self, ctx: &Context<'_>) -> User {
        ctx.data_unchecked::<User>().clone()
    }
}

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl UserMutation {
    /// Returns an authentication token if the
    /// user is found and the password matches
    async fn auth_token(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
    ) -> Result<String, anyhow::Error> {
        let conn = &mut ctx.data_unchecked::<DieselPool>().get().await?;
        let user = users::table
            .filter(users::email.eq(&email))
            .first::<User>(conn)
            .await
            .optional()?;

        match user {
            Some(user) => {
                match bcrypt::verify(&password, &user.password) {
                    Ok(true) => tracing::info!("Login Success: {}", &email),
                    Ok(false) | Err(_) => {
                        tracing::error!("Login attempt with password mismatch");
                        return Err(APIError::new("PASSWORD_MISMATCH", "Password mismatch").into());
                    }
                }

                match jsonwebtoken::encode(
                    &jsonwebtoken::Header::default(),
                    &TokenData {
                        user_id: user.id,
                        created: SystemTime::now(),
                    },
                    &EncodingKey::from_secret(dotenv::var("JWT_SECRET")?.as_bytes()),
                ) {
                    Ok(token) => Ok(token),
                    Err(_) => {
                        tracing::error!("Error creating token");
                        Err(
                            APIError::new("COULD_NOT_CREATE_TOKEN", "Could not create token")
                                .into(),
                        )
                    }
                }
            }
            None => {
                tracing::info!("Login Attempt: User not found: {}", email);
                Err(APIError::new("USER_NOT_FOUND", "User not found").into())
            }
        }
    }

    async fn create_user(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
        first_name: String,
        last_name: String,
        calling_code: String,
        country_code: String,
        phone_number: String,
        referrer: Option<Uuid>,
    ) -> Result<Uuid, anyhow::Error> {
        let conn = &mut ctx.data_unchecked::<DieselPool>().get().await?;
        let user = User {
            id: Uuid::new_v4(),
            email,
            joined: Utc::now(),
            password: match bcrypt::hash(&password, bcrypt::DEFAULT_COST) {
                Ok(hash) => hash,
                Err(e) => {
                    tracing::error!("Error hashing password: {}", e);
                    return Err(APIError::new(
                        "FAILED_TO_HASH_PASSWORD",
                        "Failed to hash password",
                    )
                    .into());
                }
            },
            first_name,
            last_name,
            calling_code,
            country_code,
            phone_number,
            referrer,
            role: None,
            stripe_customer_id: None,
        };

        match diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .await
        {
            Ok(_) => tracing::info!("User created: {}", &user.email),
            Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                tracing::error!("User already exists: {}", &user.email);
                return Err(APIError::new("USER_ALREADY_EXISTS", "User already exists").into());
            }
            Err(e) => {
                tracing::error!("Error creating user: {}", e);
                return Err(e.into());
            }
        }

        Ok(user.id)
    }
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
    async fn referral_count(&self, ctx: &Context<'_>) -> Result<i32, anyhow::Error> {
        let conn = &mut ctx.data_unchecked::<DieselPool>().get().await?;

        let count: i64 = users::table
            .filter(users::referrer.eq(self.id))
            .count()
            .get_result(conn)
            .await?;

        Ok(count as i32)
    }
    async fn citizenship_status(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<CitizenshipStatus>, anyhow::Error> {
        let conn = &mut ctx.data_unchecked::<DieselPool>().get().await?;

        let citizenship_status = applications::table
            .filter(applications::application_type.eq("citizenship"))
            .filter(applications::application.contains(&serde_json::json!({ "user_id": self.id })))
            .order_by(applications::created_at.desc())
            .first::<Application<CitizenshipApplication>>(conn)
            .await
            .optional()?;

        match citizenship_status {
            Some(Application {
                application:
                    JsonB(CitizenshipApplication {
                        citizenship_status, ..
                    }),
                ..
            }) => Ok(Some(citizenship_status)),
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
        session.return_url = return_url.as_ref().map(|url| &**url);

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
}

impl User {
    pub(crate) async fn authenticate_from_token(
        diesel: &DieselPool,
        token: String,
    ) -> Result<User, anyhow::Error> {
        // We want to use a "sliding window" token so there is no direct expiry time.
        // We will use the database to store the "last used" time of the token.
        // This means if a user constantly uses the same token within the window they will not be logged out.

        let mut validation = Validation::new(Algorithm::HS256);
        // remove default required_spec_claims
        validation.set_required_spec_claims::<&str>(&[]);
        // disable expiry valiation
        validation.validate_exp = false;

        match jsonwebtoken::decode::<TokenData>(
            &token,
            &DecodingKey::from_secret(dotenv::var("JWT_SECRET")?.as_bytes()),
            &validation,
        ) {
            Ok(decoded) => {
                let conn = &mut diesel.get().await?;
                let user = users::table
                    .find(decoded.claims.user_id)
                    .first::<User>(conn)
                    .await?;

                Ok(user)
            }
            Err(e) => {
                tracing::error!("Invalid auth token: {}", e);
                Err(APIError::new("INVALID_TOKEN", "Invalid auth token").into())
            }
        }
    }

    pub async fn stripe_customer_id(&self, ctx: &Context<'_>) -> Result<String, anyhow::Error> {
        match &self.stripe_customer_id {
            Some(customer_id) => Ok(customer_id.clone()),
            None => {
                let client = get_stripe_client();

                let customer = stripe::Customer::create(
                    &client,
                    stripe::CreateCustomer {
                        name: Some(&format!("{} {}", self.first_name, self.last_name)),
                        email: Some(&self.email),
                        ..Default::default()
                    },
                )
                .await?;

                let conn = &mut ctx.data_unchecked::<DieselPool>().get().await?;

                diesel::update(users::table.find(self.id))
                    .set(users::stripe_customer_id.eq(customer.id.to_string()))
                    .execute(conn)
                    .await?;

                Ok(customer.id.to_string())
            }
        }
    }
}
