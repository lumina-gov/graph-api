use crate::graphql::types::user::User;
use crate::schema::users;
use crate::{auth::Payload, error::APIError};

use async_graphql::{Context, Object};
use chrono::Utc;
use jsonwebtoken::EncodingKey;
use sea_orm::{
    sea_query::OnConflict, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
};
use uuid::Uuid;

#[derive(Default)]
pub struct UserMutation;

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
        let user = users::Entity::find()
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
                    &Payload { user_id: user.id },
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
            password: bcrypt::hash(&password, bcrypt::DEFAULT_COST)?,
            first_name,
            last_name,
            calling_code,
            country_code,
            phone_number,
            referrer,
            role: None,
            stripe_customer_id: None,
        };

        let active_model: users::ActiveModel = user.clone().into();

        let conn = ctx.data_unchecked::<DatabaseConnection>();

        match users::Entity::insert(active_model)
            .on_conflict(
                OnConflict::column(users::Column::Email)
                    .do_nothing()
                    .to_owned(),
            )
            .exec_with_returning(conn)
            .await
        {
            Ok(model) => {
                tracing::info!("User created: {}", &user.email);
                Ok(model.id)
            }
            Err(DbErr::RecordNotInserted) => Err(APIError::new(
                "USER_ALREADY_EXISTS",
                &format!("User already exists: {}", &user.email),
            ))?,
            Err(e) => Err(APIError::new_with_detail(
                "FAILED_TO_CREATE_USER",
                &format!("Failed to create user"),
                &format!("{:?}", e),
            ))?,
        }
    }
}
