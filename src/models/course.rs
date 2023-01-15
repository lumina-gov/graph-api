use super::schema::{courses, units};
use crate::graph::context::Context;
use diesel::BelongingToDsl;
use diesel_async::RunQueryDsl;

use diesel::{Associations, Identifiable, Insertable, Queryable};
use juniper::{graphql_object, FieldResult, GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Identifiable, Queryable, Debug)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
}

#[graphql_object(
    context = Context
)]
impl Course {
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
    pub async fn units(&self, context: &Context) -> FieldResult<Vec<Unit>> {
        let conn = &mut context.diesel_pool.get().await?;
        Ok(Unit::belonging_to(self).load::<Unit>(conn).await?)
    }
}

#[derive(GraphQLObject, Serialize, Deserialize, Identifiable, Queryable, Debug, Associations)]
#[diesel(belongs_to(Course))]
pub struct Unit {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub id: Uuid,
    pub parent_unit: Option<Uuid>,
    pub course_id: Uuid,
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
