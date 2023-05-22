use std::collections::HashMap;

use async_graphql::{Context, Object};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

use crate::{graphql::types::user::User, guards::auth::AuthGuard, schema::unit_progress};

#[derive(Default)]
pub struct UnitProgressQuery;

#[Object]
impl UnitProgressQuery {
    #[graphql(guard = "AuthGuard")]
    pub async fn course_progress(
        &self,
        ctx: &Context<'_>,
        course_slug: String,
    ) -> Result<Vec<unit_progress::Model>, anyhow::Error> {
        let user = ctx.data_unchecked::<User>();
        let conn = ctx.data_unchecked::<DatabaseConnection>();

        Ok(unit_progress::Entity::find()
            .filter(unit_progress::Column::UserId.eq(user.id))
            .filter(unit_progress::Column::CourseSlug.eq(course_slug))
            .all(conn)
            .await?)
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn all_course_progress(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<Vec<unit_progress::Model>>, anyhow::Error> {
        let user = ctx.data_unchecked::<User>();
        let conn = ctx.data_unchecked::<DatabaseConnection>();

        let mut course_progress: HashMap<String, Vec<unit_progress::Model>> = HashMap::new();

        let all_progress: Vec<unit_progress::Model> = unit_progress::Entity::find()
            .filter(unit_progress::Column::UserId.eq(user.id))
            .order_by_desc(unit_progress::Column::UpdatedAt)
            .all(conn)
            .await?;

        for progress in all_progress {
            let course_slug = progress.course_slug.clone();
            let course_progress_vec = course_progress.entry(course_slug).or_insert(vec![]);
            course_progress_vec.push(progress);
        }

        // Sort the courses by recently updated unit

        let mut vec_of_progress: Vec<_> = course_progress.into_values().collect();
        vec_of_progress.sort_by(|a, b| {
            let a = &a[0];
            let b = &b[0];
            b.updated_at.cmp(&a.updated_at)
        });

        Ok(vec_of_progress)
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn last_updated_unit(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<unit_progress::Model>, anyhow::Error> {
        let user = ctx.data_unchecked::<User>();
        let conn = ctx.data_unchecked::<DatabaseConnection>();

        Ok(unit_progress::Entity::find()
            .filter(unit_progress::Column::UserId.eq(user.id))
            .order_by_desc(unit_progress::Column::UpdatedAt)
            .one(conn)
            .await?)
    }
}
