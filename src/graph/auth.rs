use anyhow::Error;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn verifyJWT(jwt: &str) -> Result<Uuid, Error> {
    let jwt_secret = dotenv::var("JWT_SECRET").expect("JWT_SECRET not set in .env");
    let token = decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    );
    Ok(Uuid::parse_str(token?.claims.sub.as_ref())?)
}
