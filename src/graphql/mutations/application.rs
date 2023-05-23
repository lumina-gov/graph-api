use crate::guards::auth::AuthGuard;
use crate::{
    applications::validate_application, graphql::types::application::Application,
    schema::applications,
};
use async_graphql::{Context, Object};
use sea_orm::*;

#[derive(Default)]
pub struct ApplicationMutation;

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl ApplicationMutation {
    #[graphql(guard = "AuthGuard")]
    pub async fn submit_application(
        &self,
        ctx: &Context<'_>,
        application: Application,
    ) -> Result<uuid::Uuid, anyhow::Error> {
        let db = ctx.data_unchecked::<DatabaseConnection>();

        validate_application(&application).await?;

        Ok(
            applications::Entity::insert(application.into_active_model())
                .exec_with_returning(db)
                .await?
                .id,
        )
    }
}
