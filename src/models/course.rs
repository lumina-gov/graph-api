use super::schema::{course_progress, courses, units, users};
use crate::graph::context::Context;
use diesel::BelongingToDsl;
use diesel_async::RunQueryDsl;

use diesel::associations::HasTable;
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

#[derive(Serialize, Deserialize, Identifiable, Queryable, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub bio: Option<String>,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[graphql_object(
    context = Context
)]
impl User {
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn email(&self) -> String {
        self.email.clone()
    }
    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
    pub fn bio(&self) -> Option<String> {
        self.bio.clone()
    }

    // pub async fn progress(&self, context: &Context) -> FieldResult<Option<CourseProgress>> {
    //     use super::schema::course_progress::dsl::*;

    //     let conn = &mut context.diesel_pool.get().await?;
    //     let res = course_progress::table
    //         .inner_join(courses::table)
    //         .filter(user_id.eq(self.id))
    //         .load::<(CourseProgress, Course)>(conn);

    //     Ok(res.await?)
    // }
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

// PostgreSQL:
// The Queryable struct needs to be defined in the same way as it is in schema.rs,
// and field/column order needs to also be the same.
// See the following doc on how the types map: https://docs.rs/diesel/0.11.0/diesel/types/index.html.

// GraphQL:
// If we needed complex resolvers, we'd need to impl a #[graphql_object] manually, like we did for Query.
// If you need the database, remember to add context = Context.
// See the following issue for status about combining #[graphql_object] with #[derive(GraphQLObject)]:
// https://github.com/graphql-rust/juniper/issues/553
