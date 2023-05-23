use crate::{
    graphql::types::unit_progress::{
        UnitProgressActiveModel, UnitProgressColumn, UnitProgressEntity,
    },
    guards::auth::AuthGuard,
};
use async_graphql::{Context, Object};
use sea_orm::{sea_query::OnConflict, DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::graphql::types::{
    unit_progress::{UnitProgress, UnitStatus},
    user::User,
};

#[derive(Default)]
pub struct UnitProgressMutation;

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl UnitProgressMutation {
    #[graphql(guard = "AuthGuard")]
    async fn set_unit_progress(
        &self,
        ctx: &Context<'_>,
        unit_slug: String,
        course_slug: String,
        status: UnitStatus,
    ) -> async_graphql::Result<UnitProgress> {
        let user = ctx.data_unchecked::<User>();
        let conn = ctx.data_unchecked::<DatabaseConnection>();

        let unit_progress: UnitProgressActiveModel = UnitProgress {
            id: Uuid::new_v4(),
            course_slug: course_slug.clone(),
            unit_slug: unit_slug.clone(),
            user_id: user.id,
            status,
            updated_at: chrono::Utc::now(),
        }
        .into();

        Ok(UnitProgressEntity::insert(unit_progress)
            .on_conflict(
                OnConflict::columns([
                    UnitProgressColumn::UserId,
                    UnitProgressColumn::UnitSlug,
                    UnitProgressColumn::CourseSlug,
                ])
                .update_columns([UnitProgressColumn::Status, UnitProgressColumn::UpdatedAt])
                .to_owned(),
            )
            .exec_with_returning(conn)
            .await?)
    }
}
