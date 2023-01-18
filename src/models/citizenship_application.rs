use chrono::{DateTime, Utc, serde::ts_milliseconds};
use diesel_async::RunQueryDsl;
use juniper::{GraphQLInputObject, FieldResult, graphql_object};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{graph::context::UniqueContext};

use super::{applications::Application, utils::jsonb::JsonB, schema::applications};

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CitizenshipStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(GraphQLInputObject, Clone, Debug, Serialize, Deserialize)]
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
        context: &UniqueContext,
        input: CitizenshipApplicationInput,
    ) -> FieldResult<Uuid> {
        let user = context.user()?;
        let conn = &mut context.diesel_pool.get().await?;

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
        };

        diesel::insert_into(applications::table)
            .values(&application)
            .execute(conn)
            .await?;

        Ok(application.id)
    }
}

#[graphql_object(
    context = UniqueContext
)]
impl CitizenshipApplication {
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
}
