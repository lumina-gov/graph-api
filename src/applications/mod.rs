use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{error::APIError, graphql::types::application::Application};

#[derive(Serialize, Deserialize, Debug)]
struct CitizenshipApplication {
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

struct VolunteerApplication {
    pub foo: String,
}

pub async fn validate_application(app: &Application) -> Result<(), APIError> {
    match app.application_type.as_str() {
        "citizenship" => {
            serde_json::from_value::<CitizenshipApplication>(app.application.clone())
                .map(|_| ())
                .map_err(|e| APIError::new("INVALID_APPLICATION", &e.to_string()))
            // TODO also check if user is eligible for a citizenship and doesn't
        }
        _ => Err(APIError::new("INVALID_APPLICATION_TYPE", "Invalid application type").into()),
        // this shouldn't be needed after we make application into an enum
    }
}
