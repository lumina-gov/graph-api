use std::{str::FromStr, time::SystemTime};

pub use crate::schema::users::User;
use crate::{
    error::APIError,
    schema::users::{self, UserEntity},
    stripe::{get_stripe_client, stripe_search, SearchParams},
};
use async_graphql::{ComplexObject, Context, Enum, Object, SimpleObject};
use chrono::{DateTime, NaiveDateTime, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use sea_orm::{
    sea_query::OnConflict, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter,
};
use serde::{Deserialize, Serialize};
use stripe::{Application, CreateBillingPortalSession, PriceId};
use uuid::Uuid;

use super::{
    citizenship_application::{CitizenshipApplication, CitizenshipStatus},
    utils::jsonb::JsonB,
};

#[derive(Default)]
pub struct UserQuery;

#[derive(Default)]
pub struct UserMutation;

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
    async fn user_count(&self, ctx: &Context<'_>) -> Result<u64, anyhow::Error> {
        let conn = ctx.data_unchecked::<DatabaseConnection>();

        let data = UserEntity::find().count(conn).await?;

        Ok(data)
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
    ) -> Result<Vec<u64>, anyhow::Error> {
        if count > 36 {
            return Err(APIError::new("BAD_REQUEST", "Count must be below 36").into());
        }
        if interval > 6 {
            return Err(APIError::new("BAD_REQUEST", "Interval must be below 6").into());
        }

        let conn = ctx.data_unchecked::<DatabaseConnection>();

        // create a duration (interval * month)
        let duration = chrono::Duration::days(interval as i64 * 30);

        // get the current time
        let now = chrono::Utc::now();

        // for count times, get the number of users created in the last interval
        let mut data = Vec::new();

        for i in 0..count {
            let start = now - duration * i;
            let end = now - duration * (i + 1);

            let count: u64 = UserEntity::find()
                .filter(users::Column::Joined.gt(end))
                .filter(users::Column::Joined.lt(start))
                .count(conn)
                .await?;

            data.push(count);
        }

        Ok(data)
    }

    async fn me(&self, ctx: &Context<'_>) -> Option<User> {
        ctx.data_opt::<User>().cloned()
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
        let conn = ctx.data_unchecked::<DatabaseConnection>();
        let user = UserEntity::find()
            .filter(users::Column::Email.contains(&email))
            .one(conn)
            .await?;

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
        let user = User {
            id: Uuid::new_v4(),
            email,
            joined: Utc::now().into(),
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
        };

        let conn = ctx.data_unchecked::<DatabaseConnection>();

        match UserEntity::insert(user.into())
            .on_conflict(OnConflict::column(users::Column::Email))
            .exec_with_returning(conn)
        {
            Ok(_) => tracing::info!("User created: {}", &user.email),
            // Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            //     tracing::error!("User already exists: {}", &user.email);
            //     return Err(APIError::new("USER_ALREADY_EXISTS", "User already exists").into());
            // }
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
        let conn = ctx.data_unchecked::<DatabaseConnection>();

        let count: i64 = unimplemented!();
        Ok(count as i32)
    }
    async fn citizenship_status(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<CitizenshipStatus>, anyhow::Error> {
        unimplemented!()
        // let conn = ctx.data_unchecked::<DatabaseConnection>();

        // let citizenship_status = unimplemented!();
        // // applications::table
        // //     .filter(applications::application_type.eq("citizenship"))
        // //     .filter(applications::application.contains(&serde_json::json!({ "user_id": self.id })))
        // //     .order_by(applications::created_at.desc())
        // //     .first::<Application<CitizenshipApplication>>(conn)
        // //     .await
        // //     .optional()?;

        // match citizenship_status {
        //     Some(Application {
        //         application:
        //             JsonB(CitizenshipApplication {
        //                 citizenship_status, ..
        //             }),
        //         ..
        //     }) => Ok(Some(citizenship_status)),
        //     None => Ok(None),
        // }
    }

    async fn customer_portal_url(
        &self,
        return_url: Option<String>,
    ) -> Result<String, anyhow::Error> {
        let stripe_customer_id = self.stripe_customer_id().await?;
        let client = get_stripe_client();

        let mut session =
            CreateBillingPortalSession::new(stripe::CustomerId::from_str(&stripe_customer_id)?);
        session.return_url = return_url.as_deref();

        let session = stripe::BillingPortalSession::create(&client, session).await?;

        Ok(session.url)
    }

    async fn stripe_subscription_info(&self) -> Result<SubscriptionInfo, anyhow::Error> {
        let stripe_customer_id = self.stripe_customer_id().await?;
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
        diesel: &DatabaseConnection,
        token: String,
    ) -> Result<User, anyhow::Error> {
        unimplemented!()
        // // We want to use a "sliding window" token so there is no direct expiry time.
        // // We will use the database to store the "last used" time of the token.
        // // This means if a user constantly uses the same token within the window they will not be logged out.

        // let mut validation = Validation::new(Algorithm::HS256);
        // // remove default required_spec_claims
        // validation.set_required_spec_claims::<&str>(&[]);
        // // disable expiry valiation
        // validation.validate_exp = false;

        // match jsonwebtoken::decode::<TokenData>(
        //     &token,
        //     &DecodingKey::from_secret(dotenv::var("JWT_SECRET")?.as_bytes()),
        //     &validation,
        // ) {
        //     Ok(decoded) => {
        //         let conn = &mut diesel.get().await?;
        //         let user = unimplemented!();

        //         Ok(user)
        //     }
        //     Err(e) => {
        //         tracing::error!("Invalid auth token: {}", e);
        //         Err(APIError::new("INVALID_TOKEN", "Invalid auth token").into())
        //     }
        // }
    }

    pub async fn stripe_customer_id(&self) -> Result<String, anyhow::Error> {
        let client = get_stripe_client();

        match stripe_search::<stripe::Customer>(
            &client,
            "customers",
            SearchParams {
                query: format!("metadata[\"user_id\"]:\"{}\"", self.id),
                ..Default::default()
            },
        )
        .await?
        .data
        .get(0)
        {
            Some(customer) => Ok(customer.id.to_string()),
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

                Ok(customer.id.to_string())
            }
        }
    }
}
