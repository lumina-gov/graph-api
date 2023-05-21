use async_graphql::Context;
use chrono::Utc;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::schema::{sea_orm_active_enums::UnitStatus, unit_progress::UnitProgress};

use super::user::User;

impl UnitProgress {
    fn new(user_id: Uuid, unit_slug: String, course_slug: String, status: UnitStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            unit_slug,
            course_slug,
            status,
            updated_at: Utc::now().into(),
        }
    }

    pub async fn create_or_update(
        ctx: &Context<'_>,
        user: &User,
        unit_slug: String,
        course_slug: String,
        status: UnitStatus,
    ) -> Result<Self, anyhow::Error> {
        let conn = ctx.data_unchecked::<DatabaseConnection>();
        let unit_progress = Self::new(user.id, unit_slug, course_slug, status);

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

    pub async fn course_progress(
        ctx: &Context<'_>,
        user: &User,
        course_slug: String,
    ) -> Result<Vec<Self>, anyhow::Error> {
        unimplemented!()
        // let conn = &mut ctx.data_unchecked::<DieselPool>().get().await?;
        // match unit_progress::table
        //     .filter(unit_progress::user_id.eq(user.id))
        //     .filter(unit_progress::course_slug.eq(course_slug))
        //     .get_results(conn)
        //     .await
        // {
        //     Ok(unit_progress) => Ok(unit_progress),
        //     Err(e) => Err(e.into()),
        // }
    }

    pub async fn all_course_progress(
        ctx: &Context<'_>,
        user: &User,
    ) -> Result<Vec<Vec<Self>>, anyhow::Error> {
        unimplemented!()
        // let conn = &mut ctx.data_unchecked::<DieselPool>().get().await?;
        // // We want to get all the Unit progresses for a user, but group them by the course_slug
        // // so we can return a Vec<Vec<Self>> where each inner Vec<Self> is a course
        // // and each Self is a UnitProgress

        // let mut course_progress: HashMap<String, Vec<Self>> = HashMap::new();
        // // order by updated_at desc so that the most recently updated unit is first
        // let all_progress: Vec<Self> = unit_progress::table
        //     .order_by(unit_progress::updated_at.desc())
        //     .filter(unit_progress::user_id.eq(user.id))
        //     .get_results(conn)
        //     .await?;

        // for progress in all_progress {
        //     let course_slug = progress.course_slug.clone();
        //     let course_progress_vec = course_progress.entry(course_slug).or_insert(vec![]);
        //     course_progress_vec.push(progress);
        // }

        // //Allows the course to be sorted to the last unit touched instead of random hashmap, done it this way to avoid rust compliler complaining
        // let mut vec_of_progress: Vec<_> = course_progress.into_values().collect();
        // vec_of_progress.sort_by(|a, b| b[0].updated_at.cmp(&a[0].updated_at));
        // Ok(vec_of_progress)
    }

    pub async fn last_updated_unit(
        ctx: &Context<'_>,
        user: &User,
    ) -> Result<Option<Self>, anyhow::Error> {
        unimplemented!()
        // let conn = &mut ctx.data_unchecked::<DieselPool>().get().await?;
        // Ok(unit_progress::table
        //     .filter(unit_progress::user_id.eq(user.id))
        //     .order(unit_progress::updated_at.desc())
        //     .first(conn)
        //     .await
        //     .optional()?)
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize, Enum, DbEnum, Copy, Eq, PartialEq)]
// #[ExistingTypePath = "crate::db_schema::sql_types::UnitStatus"]
// #[DbValueStyle = "PascalCase"]
// pub enum UnitStatus {
//     NotStarted,
//     InProgress,
//     Completed,
// }
