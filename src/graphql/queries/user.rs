use crate::{error::new_err, graphql::types::user::User, schema::users};
use async_graphql::{Context, Object};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};

#[derive(Default)]
pub struct UserQuery;

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl UserQuery {
    async fn user_count(&self, ctx: &Context<'_>) -> async_graphql::Result<u64> {
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
    ) -> async_graphql::Result<Vec<u64>> {
        if count > 36 {
            return Err(new_err("BAD_REQUEST", "Count must be below 36").into());
        }
        if interval > 6 {
            return Err(new_err("BAD_REQUEST", "Interval must be below 6").into());
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

            let count: u64 = users::Entity::find()
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
