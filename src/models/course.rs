use juniper::{GraphQLObject};
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::models::schema::courses;
use uuid::Uuid;

#[derive(GraphQLObject, Serialize, Deserialize, Queryable, Insertable, Debug)]
pub struct Course {
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
}