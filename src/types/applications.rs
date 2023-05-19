use crate::db_schema::applications;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::utils::jsonb::JsonB;

#[derive(Clone, Debug, Serialize, Deserialize, Identifiable, Queryable, Insertable)]
pub struct Application<T> {
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub application: JsonB<T>,
    pub application_type: String,
}
