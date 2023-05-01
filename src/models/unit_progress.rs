use std::collections::HashMap;

use crate::{graph::context::UniqueContext, models::schema::unit_progress};
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, ExpressionMethods, QueryDsl, Associations, OptionalExtension};
use diesel_async::RunQueryDsl;
use diesel_derive_enum::DbEnum;
use juniper::{GraphQLEnum, GraphQLObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user::User;

#[derive(GraphQLObject, Debug, Clone, Deserialize, Serialize, Identifiable, Queryable, Insertable, Associations)]
#[diesel(table_name = unit_progress)]
#[diesel(belongs_to(User))]
#[graphql(rename_all = "none")]
pub struct UnitProgress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub unit_slug: String,
    pub course_slug: String,
    pub status: UnitStatus,
    #[serde(with = "ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
}

impl UnitProgress {
    fn new(user_id: Uuid, unit_slug: String, course_slug: String, status: UnitStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            unit_slug,
            course_slug,
            status,
            updated_at: Utc::now(),
        }
    }

    pub async fn create_or_update(
        context: &UniqueContext,
        user: &User,
        unit_slug: String,
        course_slug: String,
        status: UnitStatus,
    ) -> Result<Self, anyhow::Error> {
        let conn = &mut context.diesel_pool.get().await?;
        match diesel::insert_into(unit_progress::table)
            .values(Self::new(user.id, unit_slug, course_slug, status.clone()))
            .on_conflict((unit_progress::user_id, unit_progress::unit_slug, unit_progress::course_slug))
            .do_update()
            .set((
                unit_progress::status.eq(status),
                unit_progress::updated_at.eq(Utc::now()),
            ))
            .get_result(conn)
            .await
        {
            Ok(unit_progress) => Ok(unit_progress),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn course_progres(
        context: &UniqueContext,
        user: &User,
        course_slug: String,
    ) -> Result<Vec<Self>, anyhow::Error> {
        let conn = &mut context.diesel_pool.get().await?;
        match unit_progress::table
            .filter(unit_progress::user_id.eq(user.id))
            .filter(unit_progress::course_slug.eq(course_slug))
            .get_results(conn)
            .await
        {
            Ok(unit_progress) => Ok(unit_progress),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn all_course_progress(
        context: &UniqueContext,
        user: &User
    ) -> Result<Vec<Vec<Self>>, anyhow::Error> {
        let conn = &mut context.diesel_pool.get().await?;
         // We want to get all the Unit progresses for a user, but group them by the course_slug
        // so we can return a Vec<Vec<Self>> where each inner Vec<Self> is a course
        // and each Self is a UnitProgress

        let mut course_progress: HashMap<String, Vec<Self>> = HashMap::new();
        // order by updated_at desc so that the most recently updated unit is first
        let all_progress: Vec<Self> = unit_progress::table
            .order_by(unit_progress::updated_at.desc())
            .filter(unit_progress::user_id.eq(user.id))
            .get_results(conn)
            .await?;

        for progress in all_progress {
            let course_slug = progress.course_slug.clone();
            let course_progress_vec = course_progress.entry(course_slug).or_insert(vec![]);
            course_progress_vec.push(progress);
        }

        Ok(course_progress.into_values().collect())
    }

    pub async fn last_updated_unit(context: &UniqueContext, user: &User) -> Result<Option<Self>, anyhow::Error> {
        let conn = &mut context.diesel_pool.get().await?;
        Ok(unit_progress::table
            .filter(unit_progress::user_id.eq(user.id))
            .order(unit_progress::updated_at.desc())
            .first(conn)
            .await
            .optional()?)
    }
}

#[derive(
    Debug, Clone, Serialize, Deserialize, GraphQLEnum, DbEnum
)]
#[ExistingTypePath = "crate::models::schema::sql_types::UnitStatus"]
#[DbValueStyle = "PascalCase"]
pub enum UnitStatus {
    NotStarted,
    InProgress,
    Completed,
}
