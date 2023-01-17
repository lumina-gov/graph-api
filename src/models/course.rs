use crate::graph::context::UniqueContext;

use super::{
    schema::{course_progress, courses, units, users},
    unit::Unit,
};
use diesel::BelongingToDsl;
use diesel_async::RunQueryDsl;

use diesel::{Identifiable, Insertable, Queryable};
use juniper::{graphql_object, FieldResult, GraphQLInputObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Identifiable, Queryable, Debug)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub slug: String,
}

#[graphql_object(
    context = UniqueContext
)]
impl Course {
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn slug(&self) -> String {
        self.slug.clone()
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
    pub async fn units(&self, context: &UniqueContext) -> FieldResult<Vec<Unit>> {
        let conn = &mut context.diesel_pool.get().await?;
        Ok(Unit::belonging_to(self).load::<Unit>(conn).await?)
    }
}

// #[derive(Serialize, Deserialize, Identifiable, Queryable, Debug)]
// struct CourseProgress {
//     id: i32,
//     course_id: Uuid,
//     user_id: Uuid,
//     credits: i32,
// }

// #[derive(GraphQLObject, Serialize, Deserialize, Identifiable, Queryable, Debug, Associations)]
// #[diesel(belongs_to(Course))]
// #[diesel(belongs_to(User))]
// pub struct CourseProgress {
//     pub id: i32,
//     pub credits: i32,
//     pub course: Course,
// }

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
