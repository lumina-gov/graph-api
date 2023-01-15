use super::schema::courses;

use diesel::{Insertable, Queryable};
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(GraphQLObject, Serialize, Deserialize, Queryable, Debug)]
pub struct Course {
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug)]
pub struct CreateCourseInput {
    pub name: String,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = courses)]
pub struct CourseInsertable {
    pub name: String,
    pub id: Uuid,
}
