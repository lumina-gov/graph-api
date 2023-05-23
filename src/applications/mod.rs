use async_graphql::Enum;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{error::new_err, graphql::types::application::Application};

#[derive(Serialize, Deserialize, Debug)]
pub struct CitizenshipApplication {
    pub user_id: uuid::Uuid,
    #[serde(with = "chrono::serde::ts_milliseconds")]
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

pub async fn validate_application(app: &Application) -> async_graphql::Result<()> {
    match app.application_type.as_str() {
        "citizenship" => {
            serde_json::from_value::<CitizenshipApplication>(app.application.clone())
                .map(|_| ())
                .map_err(|e| new_err("INVALID_APPLICATION", &e.to_string()))
            // TODO also check if user is eligible for a citizenship and doesn't
        }
        _ => Err(new_err(
            "INVALID_APPLICATION_TYPE",
            "Invalid application type",
        )),
        // this shouldn't be needed after we make application into an enum
    }
}
