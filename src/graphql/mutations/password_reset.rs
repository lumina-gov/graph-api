use std::str::FromStr;

use lazy_static::lazy_static;

use async_graphql::{Context, Object};
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
};
use sendgrid::SGClient;
use tracing::{event, Level};

use crate::{
    error::new_err,
    graphql::types::Void,
    schema::{self, password_reset_tokens, users},
};

lazy_static! {
    static ref RESET_URL_BASE: url::Url = url::Url::from_str("https://lumina.earth/reset").unwrap();
}
#[derive(Default)]
pub struct PasswordResetMutation;

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl PasswordResetMutation {
    #[graphql()]
    pub async fn reset_password(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> async_graphql::Result<Void> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let s_g_client = ctx.data_unchecked::<SGClient>();

        let user = users::Entity::find()
            .filter(users::Column::Email.contains(&email))
            .one(db)
            .await?
            .ok_or_else(|| new_err("USER_NOT_FOUND", &format!("User not found: {}", &email)))?;

        let new_reset_token = password_reset_tokens::Model {
            id: uuid::Uuid::new_v4(),
            user_id: user.id,
        };

        let token = match password_reset_tokens::Entity::insert(new_reset_token.into_active_model())
            .exec_with_returning(db)
            .await
        {
            Err(db_error) => {
                event!(Level::ERROR, "{}", db_error);
                return Err(new_err(
                    "TOKEN_CREATION_ERROR",
                    "unable to create new token",
                ));
            }

            Ok(token) => token.id,
        };
        let mut reset_url = RESET_URL_BASE.to_owned();
        reset_url
            .query_pairs_mut()
            .append_pair("token", &token.as_simple().to_string())
            .append_pair("email", &email);
        let reset_text = format!("go to {} to reset your password", reset_url.to_string());
        let reset_password_mail = sendgrid::Mail::new()
            .add_from("no-reply@lumina.earth")
            .add_text(&reset_text)
            .add_subject("Lumina: Your password reset link!")
            .add_to(sendgrid::Destination {
                address: &email,
                name: &user.first_name,
            });
        match s_g_client.send(reset_password_mail).await {
            Ok(_) => return Ok(Void),
            Err(error) => {
                event!(Level::ERROR, "{}", error);
                return Err(new_err("EMAIL_SEND_ERROR", "unable to send email"));
            }
        }
    }
    #[graphql()]
    pub async fn reset_to_new_password(
        &self,
        ctx: &Context<'_>,
        token_id: uuid::Uuid,
        new_password: String,
    ) -> async_graphql::Result<Void> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        if new_password.len() < 8 {
            return Err(new_err(
                "PASSWORD_TOO_SHORT",
                "Your password needs to be at least 8 characters long",
            ));
        }
        let user_token = schema::password_reset_tokens::Entity::find_by_id(token_id)
            .one(db)
            .await?
            .ok_or_else(|| new_err("TOKEN_NOT_FOUND", "token doesn't exist"))?;

        let _ = schema::password_reset_tokens::Entity::delete_by_id(user_token.id)
            .exec(db)
            .await;

        let user = users::Entity::find_by_id(user_token.user_id)
            .one(db)
            .await?
            .ok_or_else(|| {
                new_err(
                    "USER_NOT_FOUND",
                    &format!("User not found: {}", user_token.id),
                )
            })?;
        let mut active_user = user.into_active_model();
        let hashed_password = bcrypt::hash(&new_password, bcrypt::DEFAULT_COST)?;
        active_user.password = ActiveValue::Set(hashed_password);

        match users::Entity::update(active_user).exec(db).await {
            Ok(_) => Ok(Void),
            Err(error) => {
                event!(Level::ERROR, "{}", error);
                return Err(new_err(
                    "PASSWORD_CHANGE_ERROR",
                    "unable to change password",
                ));
            }
        }
    }
}
