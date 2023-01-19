use diesel::{Identifiable, Queryable, Insertable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::models::schema::applications;

use super::utils::jsonb::JsonB;

#[derive(Clone, Debug, Serialize, Deserialize, Identifiable, Queryable, Insertable)]
pub struct Application<T> {
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub application: JsonB<T>,
    pub application_type: String,
}
