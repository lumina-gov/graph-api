use crate::applications::{CitizenshipApplication, CitizenshipStatus};
use crate::error::new_err;
use crate::graphql::types::user::User;
use crate::guards::auth::AuthGuard;
use crate::{
    applications::validate_application, graphql::types::application::Application,
    schema::applications,
};
use async_graphql::{Context, Object};
use chrono::{DateTime, Utc};
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel};

#[derive(Default)]
pub struct ApplicationMutation;

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl ApplicationMutation {
    #[graphql(guard = "AuthGuard")]
    pub async fn submit_application(
        &self,
        ctx: &Context<'_>,
        application: Application,
    ) -> async_graphql::Result<uuid::Uuid> {
        let db = ctx.data_unchecked::<DatabaseConnection>();

        validate_application(&application).await?;

        Ok(
            applications::Entity::insert(application.into_active_model())
                .exec_with_returning(db)
                .await?
                .id,
        )
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn create_citizenship_application(
        &self,
        ctx: &Context<'_>,
        // Unix timestamp in milliseconds
        date_of_birth: u64,
        sex: String,
        first_name: String,
        last_name: String,
        skills: Vec<String>,
        occupations: Vec<String>,
        country_of_citizenship: Vec<String>,
        country_of_birth: String,
        country_of_residence: String,
        ethnic_groups: Vec<String>,
    ) -> async_graphql::Result<uuid::Uuid> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let user = ctx.data_unchecked::<User>();

        let citizenship_application = CitizenshipApplication {
            user_id: user.id,
            citizenship_status: CitizenshipStatus::Pending,
            country_of_birth,
            country_of_citizenship,
            country_of_residence,
            ethnic_groups,
            first_name,
            last_name,
            occupations,
            skills,
            sex,
            date_of_birth: DateTime::<Utc>::from_utc(
                chrono::NaiveDateTime::from_timestamp_millis(date_of_birth as i64)
                    .ok_or_else(|| new_err("INVALID_DATE", "Invalid date of birth provided"))?,
                Utc,
            ),
        };

        let application = Application {
            application: serde_json::to_value(citizenship_application)?,
            application_type: "citizenship".to_string(),
            created_at: chrono::Utc::now(),
            id: uuid::Uuid::new_v4(),
        };

        Ok(
            applications::Entity::insert(application.into_active_model())
                .exec_with_returning(db)
                .await?
                .id,
        )
    }
}
