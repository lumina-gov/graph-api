use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct AuthApp {
    pub name: String,
    pub description: String,
    pub created: DateTime<Utc>,
    pub redirect_hostnames: Vec<String>,
    pub scopes: Vec<String>,
    pub official: bool,
}
