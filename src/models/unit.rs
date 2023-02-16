use diesel::{Identifiable, Queryable, Associations};
use juniper::GraphQLObject;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::models::schema::units;
use crate::models::course::Course;


#[derive(GraphQLObject, Serialize, Deserialize, Identifiable, Queryable, Debug, Associations)]
#[diesel(belongs_to(Course))]
#[graphql(rename_all = "none")]
pub struct Unit {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub id: Uuid,
    pub parent_unit: Option<Uuid>,
    pub course_id: Uuid,
    pub slug : String,
    pub notion_page_id: Option<String>,
}