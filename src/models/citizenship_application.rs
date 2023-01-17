use chrono::{DateTime, Utc, serde::ts_milliseconds};
use diesel::{Identifiable, Queryable, Insertable};
use juniper::GraphQLInputObject;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{models::schema::citizenship_applications};
#[derive(Clone, Debug, Serialize, Deserialize, Identifiable, Queryable, Insertable)]
pub struct CitizenshipApplication {
    pub user_id: Uuid,
    #[serde(with = "ts_milliseconds")]
    pub submitted_date: DateTime<Utc>,
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
    pub id: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::models::schema::sql_types::CitizenshipStatus"]
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