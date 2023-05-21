#[derive(Default)]
pub struct UnitProgressQuery;

#[Object]
impl UnitProgressQuery {
    pub async fn course_progress(
        &self,
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
        &self,
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
        &self,
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
