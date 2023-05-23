use crate::{error::APIError, graphql::types::user::User, schema::users};
use async_graphql::{Context, Object};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};

#[derive(Default)]
pub struct UserQuery;

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl UserQuery {
    async fn user_count(&self, ctx: &Context<'_>) -> Result<u64, anyhow::Error> {
        let conn = ctx.data_unchecked::<DatabaseConnection>();

        let data = users::Entity::find().count(conn).await?;

        Ok(data)
    }

    // Get the number of users grouped by their creation date, for a specified interval
    // in months, and the count of intervals to go back in time.
    // For example, if interval is 1 and count is 12, this will return the number of users
    // created in the last 12 months, grouped by month.
    async fn user_count_by_interval(
        &self,
        ctx: &Context<'_>,
        interval: i32,
        count: i32,
    ) -> Result<Vec<u64>, anyhow::Error> {
        if count > 36 {
            return Err(APIError::new("BAD_REQUEST", "Count must be below 36").into());
        }
        if interval > 6 {
            return Err(APIError::new("BAD_REQUEST", "Interval must be below 6").into());
        }

        let conn = ctx.data_unchecked::<DatabaseConnection>();

        // create a duration (interval * month)
        let duration = chrono::Duration::days(interval as i64 * 30);

        // get the current time
        let now = chrono::Utc::now();

        // for count times, get the number of users created in the last interval
        let mut data = Vec::new();

        for i in 0..count {
            let start = now - duration * i;
            let end = now - duration * (i + 1);

            let count: u64 = users::Entity::find()
                .filter(users::Column::Joined.gt(end))
                .filter(users::Column::Joined.lt(start))
                .count(conn)
                .await?;

            data.push(count);
        }

        Ok(data)
    }

    async fn me(&self, ctx: &Context<'_>) -> Option<User> {
        ctx.data_opt::<User>().cloned()
    }
}
