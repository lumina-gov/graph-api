use async_graphql::{async_trait::async_trait, Guard, Context, Result};

use crate::{error::APIError, types::user::User};



pub(crate) struct AuthGuard;

#[async_trait]
impl Guard for AuthGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        match ctx.data_opt::<User>() {
            Some(_) => Ok(()),
            None => Err(APIError::new("UNAUTHENTICATED", "You must be logged in to perform this action").into()),
        }
    }
}