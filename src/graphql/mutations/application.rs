use crate::guards::auth::AuthGuard;
use crate::{
    applications::validate_application, error::APIError, graphql::types::application::Application,
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
    ) -> Result<bool, APIError> {
        let db = ctx.data_unchecked::<DatabaseConnection>();

        if let Err(e) = validate_application(&application).await {
            Err(e)
        } else {
            applications::Entity::insert(application.into_active_model())
                .exec(db)
                .await
                .map_err(|e| APIError::new("INSERT_ERROR", "Couldn't insert value into database"))
                .map(|_| true)
            // todo make this error conversion automatic
        }
    }
}
