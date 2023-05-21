use crate::guards::auth::AuthGuard;
use async_graphql::{Context, Object};

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
    ) -> Result<UnitProgress, anyhow::Error> {
        let user = ctx.data_unchecked::<User>();

        unimplemented!()
        // match UnitProgressEntity::insert(unit_progress).on_conflict(
        //     seq_query::OnConflict::
        // ) {}

        // match diesel::insert_into(unit_progress::table)
        //     .values(Self::new(user.id, unit_slug, course_slug, status))
        //     .on_conflict((
        //         unit_progress::user_id,
        //         unit_progress::unit_slug,
        //         unit_progress::course_slug,
        //     ))
        //     .do_update()
        //     .set((
        //         unit_progress::status.eq(status),
        //         unit_progress::updated_at.eq(Utc::now()),
        //     ))
        //     .get_result(conn)
        //     .await
        // {
        //     Ok(unit_progress) => Ok(unit_progress),
        //     Err(e) => Err(e.into()),
        // }
    }
}
