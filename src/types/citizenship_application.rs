use async_graphql::{Context, Enum, InputObject, Object};
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db_schema::applications;
use crate::DieselPool;

use super::{applications::Application, user::User, utils::jsonb::JsonB};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CitizenshipApplication {
    pub user_id: Uuid,
    #[serde(with = "ts_milliseconds")]
    pub date_of_birth: DateTime<Utc>,
    pub sex: String,
    pub first_name: String,
    pub last_name: String,
    pub skills: Vec<String>,
    pub occupations: Vec<String>,
    pub country_of_citizenship: Vec<String>,
    pub country_of_birth: String,
    pub country_of_residence: String,
    pub ethnic_groups: Vec<String>,
    pub citizenship_status: CitizenshipStatus,
}

#[derive(Enum, Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum CitizenshipStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
#[graphql(rename_fields = "snake_case")]
pub struct CitizenshipApplicationInput {
    #[serde(with = "ts_milliseconds")]
    pub date_of_birth: DateTime<Utc>,
    pub sex: String,
    pub first_name: String,
    pub last_name: String,
    pub skills: Vec<String>,
    pub occupations: Vec<String>,
    pub country_of_citizenship: Vec<String>,
    pub country_of_birth: String,
    pub country_of_residence: String,
    pub ethnic_groups: Vec<String>,
}

impl CitizenshipApplication {
    pub async fn create_citizenship_application(
        ctx: &Context<'_>,
        input: CitizenshipApplicationInput,
    ) -> Result<Uuid, anyhow::Error> {
        let user = ctx.data_unchecked::<User>();
        let conn = &mut ctx.data_unchecked::<DieselPool>().get().await?;

        let citizenship_application = CitizenshipApplication {
            citizenship_status: CitizenshipStatus::Pending,
            user_id: user.id,
            date_of_birth: input.date_of_birth,
            first_name: input.first_name,
            last_name: input.last_name,
            skills: input.skills,
            occupations: input.occupations,
            country_of_citizenship: input.country_of_citizenship,
            country_of_birth: input.country_of_birth,
            country_of_residence: input.country_of_residence,
            ethnic_groups: input.ethnic_groups,
            sex: input.sex,
        };

        let application = Application {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            application: JsonB(citizenship_application),
            application_type: "citizenship".to_string(),
        };

        diesel::insert_into(applications::table)
            .values(&application)
            .execute(conn)
            .await?;

        Ok(application.id)
    }
}

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl CitizenshipApplication {
    async fn user_id(&self) -> Uuid {
        self.user_id
    }
}
