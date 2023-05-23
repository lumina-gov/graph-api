use crate::error::APIError;
use crate::graphql::types::user::User;
use crate::schema::users;
use anyhow::anyhow;
use anyhow::Result;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use lambda_http::Request;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Payload {
    pub user_id: Uuid,
}

pub async fn authenticate_token(db: &DatabaseConnection, token: &str) -> Result<User> {
    match jsonwebtoken::decode::<Payload>(
        token,
        &DecodingKey::from_secret(std::env::var("JWT_SECRET")?.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(payload) => {
            let user = users::Entity::find_by_id(payload.claims.user_id)
                .one(db)
                .await?;
            user.ok_or(anyhow!("Invalid user id"))
        }
        Err(e) => {
            tracing::error!("Invalid auth token: {}", e);
            Err(APIError::new("INVALID_TOKEN", "Invalid auth token").into())
        }
    }
}

pub async fn authenticate_request(db: &DatabaseConnection, event: Request) -> Result<User> {
    let header = event.headers().get("Authorization");

    if let Some(header) = header.and_then(|h| h.to_str().ok()) {
        if let Some(("", token)) = header.split_once("Bearer ") {
            authenticate_token(db, token).await
        } else {
            Err(anyhow!("Invalid Authorization token, must be Bearer"))
        }
    } else {
        Err(anyhow!("No Authorization header"))
    }
}
