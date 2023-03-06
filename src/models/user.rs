use std::str::FromStr;
use std::time::SystemTime;

use crate::error::ErrorCode;
use crate::graph::context::UniqueContext;
use crate::models::schema::users;
use crate::stripe::get_stripe_client;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, PgJsonbExpressionMethods};
use diesel::{
    result::DatabaseErrorKind, Identifiable, Insertable, OptionalExtension, QueryDsl, Queryable,
};
use diesel_async::RunQueryDsl;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use juniper::{graphql_object, FieldResult, GraphQLInputObject};
use serde::{Deserialize, Serialize};
use stripe::CreateBillingPortalSession;
use uuid::Uuid;

use super::applications::Application;
use super::citizenship_application::{CitizenshipStatus, CitizenshipApplication};
use super::utils::jsonb::JsonB;

#[derive(Debug, Clone, Deserialize, Serialize, Identifiable, Queryable, Insertable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(with = "ts_milliseconds")]
    pub joined: DateTime<Utc>,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub calling_code: String,
    pub country_code: String,
    pub phone_number: String,
    pub role: Option<String>,
    pub object_id: Option<String>,
    pub referrer: Option<Uuid>,
    pub referrer_mongo: Option<String>,
    pub stripe_customer_id: Option<String>,
}

#[graphql_object(
    context = UniqueContext,
    rename_all = "none"
)]
impl User {
    fn id(&self) -> Uuid {
        self.id
    }
    fn email(&self) -> String {
        self.email.clone()
    }
    fn first_name(&self) -> String {
        self.first_name.clone()
    }
    fn last_name(&self) -> String {
        self.last_name.clone()
    }
    fn roles(&self) -> Vec<String> {
        //TODO frontend wants an array of roles, so implement as an array
        match &self.role {
            Some(role) => vec![role.clone()],
            None => vec![],
        }
    }
    async fn referral_count(&self, context: &UniqueContext) -> FieldResult<i32> {
        let conn = &mut context.diesel_pool.get().await?;

        let count: i64 = users::table
            .filter(users::referrer.eq(self.id))
            .count()
            .get_result(conn)
            .await?;

        Ok(count as i32)
    }
    async fn citizenship_status(&self, context: &UniqueContext) -> FieldResult<Option<CitizenshipStatus>> {
        use super::schema::applications::dsl::*;

        let conn = &mut context.diesel_pool.get().await?;

        let citizenship_status = applications
            .filter(application_type.eq("citizenship"))
            .filter(application.contains(&serde_json::json!({ "user_id": self.id })))
            .order_by(created_at.desc())
            .first::<Application<CitizenshipApplication>>(conn)
            .await
            .optional()?;

        match citizenship_status {
            Some(Application {
                application: JsonB(CitizenshipApplication { citizenship_status, .. }),
                ..
            }) => Ok(Some(citizenship_status)),
            None => Ok(None),
        }
    }

    async fn customer_portal_url(&self, context: &UniqueContext, return_url: Option<String>) -> FieldResult<String> {
        let stripe_customer_id = self.stripe_customer_id(context).await?;
        let client = get_stripe_client();

        let mut session = CreateBillingPortalSession::new(stripe::CustomerId::from_str(&stripe_customer_id)?);
        session.return_url = return_url.as_ref().map(|url| &**url);

        let session = stripe::BillingPortalSession::create(&client, session).await?;

        Ok(session.url)
    }
}

impl User {
    pub async fn create_user(
        context: &UniqueContext,
        create_user: CreateUserInput,
    ) -> FieldResult<Uuid> {
        let conn = &mut context.diesel_pool.get().await?;
        let user = User {
            id: Uuid::new_v4(),
            email: create_user.email,
            joined: Utc::now(),
            password: match bcrypt::hash(&create_user.password, bcrypt::DEFAULT_COST) {
                Ok(hash) => hash,
                Err(e) => {
                    tracing::error!("Error hashing password: {}", e);
                    return Err(ErrorCode::FailedToHashPassword.into());
                }
            },
            first_name: create_user.first_name,
            last_name: create_user.last_name,
            calling_code: create_user.calling_code,
            country_code: create_user.country_code,
            phone_number: create_user.phone_number,
            referrer: None,
            object_id: None,
            referrer_mongo: None,
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
                return Err(ErrorCode::UserAlreadyExists.into());
            }
            Err(e) => {
                tracing::error!("Error creating user: {}", e);
                return Err(e.into());
            }
        }

        Ok(user.id)
    }

    /// Returns an authentication token if the
    /// user is found and the password matches
    pub async fn login_user(
        context: &UniqueContext,
        login_user: LoginUserInput,
    ) -> FieldResult<String> {
        let conn = &mut context.diesel_pool.get().await?;
        let user = users::table
            .filter(users::email.eq(&login_user.email))
            .first::<User>(conn)
            .await
            .optional()?;

        match user {
            Some(user) => {
                match bcrypt::verify(&login_user.password, &user.password) {
                    Ok(true) => tracing::info!("Login Success: {}", &login_user.email),
                    Ok(false) | Err(_) => {
                        tracing::error!("Login attempt with password mismatch");
                        return Err(ErrorCode::PasswordMismatch.into());
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
                        Err(ErrorCode::CouldNotCreateToken.into())
                    }
                }
            }
            None => {
                tracing::info!("Login Attempt: User not found: {}", login_user.email);
                Err(ErrorCode::UserNotFound.into())
            }
        }
    }

    pub async fn authenticate_from_token(
        context: &UniqueContext,
        token: String,
    ) -> FieldResult<User> {
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
                let conn = &mut context.diesel_pool.get().await?;
                let user = users::table
                    .find(decoded.claims.user_id)
                    .first::<User>(conn)
                    .await?;

                Ok(user)
            }
            Err(e) => {
                tracing::error!("Invalid auth token: {}", e);
                Err(ErrorCode::InvalidToken.into())
            }
        }
    }

    pub async fn stripe_customer_id(
        &self,
        context: &UniqueContext,
    ) -> FieldResult<String> {
        match &self.stripe_customer_id {
            Some(customer_id) => Ok(customer_id.clone()),
            None => {
                let client = get_stripe_client();

                let customer = stripe::Customer::create(&client, stripe::CreateCustomer {
                    name: Some(&format!("{} {}", self.first_name, self.last_name)),
                    email: Some(&self.email),
                    ..Default::default()
                }).await?;

                let conn = &mut context.diesel_pool.get().await?;

                diesel::update(users::table.find(self.id))
                    .set(users::stripe_customer_id.eq(customer.id.to_string()))
                    .execute(conn)
                    .await?;

                Ok(customer.id.to_string())
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, GraphQLInputObject)]
#[graphql(rename_all = "none")]
pub struct CreateUserInput {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub calling_code: String,
    pub country_code: String,
    pub phone_number: String,
    pub referrer: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, GraphQLInputObject)]
#[graphql(rename_all = "none")]
pub struct LoginUserInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenData {
    user_id: Uuid,
    created: SystemTime,
}
