use crate::error::APIError;
use crate::graphql::types::user::User;
use crate::schema::users;
use chrono::DateTime;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use lambda_http::Request;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct TokenPayload {
    pub user_id: Uuid,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created: DateTime<Utc>,
}

pub async fn authenticate_token(
    db: &DatabaseConnection,
    token: &str,
) -> Result<User, anyhow::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false;
    validation.set_required_spec_claims::<&str>(&[]);

    let payload = jsonwebtoken::decode::<TokenPayload>(
        token,
        &DecodingKey::from_secret(std::env::var("JWT_SECRET")?.as_bytes()),
        &validation,
    )
    .map_err(|_| APIError::new("INVALID_TOKEN", "Invalid auth token"))?
    .claims;

    let user = users::Entity::find_by_id(payload.user_id)
        .one(db)
        .await?
        .ok_or_else(|| APIError::new("INVALID_TOKEN", "User does not exist"))?;

    Ok(user)
}

pub async fn authenticate_request(
    db: &DatabaseConnection,
    event: Request,
) -> Result<Option<User>, anyhow::Error> {
    let header = event.headers().get("Authorization");

    if let Some(header) = header.and_then(|h| h.to_str().ok()) {
        if let Some(token) = header.strip_prefix("Bearer ") {
            Ok(Some(authenticate_token(db, token).await?))
        } else {
            Err(APIError::new("INVALID_TOKEN", "Auth header should use Bearer").into())
        }
    } else {
        Ok(None)
    }
}