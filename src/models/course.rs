use crate::{graph::context::UniqueContext, error::ErrorCode};

use super::{
    schema::{courses, enrollments},
    unit::Unit, enrollments::Enrollment,
};
use diesel::{BelongingToDsl, QueryDsl};
use diesel_async::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::{Identifiable, Queryable};
use diesel::OptionalExtension;
use juniper::{graphql_object, FieldResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Identifiable, Queryable, Debug)]
pub struct Course {
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub slug: String,
}

#[graphql_object(
    context = UniqueContext
    rename_all = "none"
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

    // Is the user enrolled in this course?
    // This will return an error if the user is not logged in
    pub async fn is_enrolled(&self, context: &UniqueContext) -> FieldResult<bool> {
        let user = match &context.user {
            Some(user) => user,
            None => return Err(ErrorCode::Unauthenticated.into()),
        };

        let conn = &mut context.diesel_pool.get().await?;

        let enrollment = enrollments::table
            .filter(enrollments::user_id.eq(user.id))
            .filter(enrollments::course_id.eq(self.id))
            .first::<Enrollment>(conn)
            .await
            .optional()?;

        Ok(enrollment.is_some())
    }
}