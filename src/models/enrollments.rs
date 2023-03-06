use diesel::{Identifiable, Queryable, Associations, Insertable};
use diesel_async::RunQueryDsl;
use juniper::FieldResult;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{models::schema::enrollments, graph::context::UniqueContext, error::ErrorCode};

use super::{course::Course, user::User};

#[derive(Serialize, Deserialize, Associations, Identifiable, Insertable, Queryable, Debug)]
#[belongs_to(Course)]
#[belongs_to(User)]
pub struct Enrollment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub enrolled_date: chrono::DateTime<chrono::Utc>,
}

impl Enrollment {
    pub async fn enroll_user(context: &UniqueContext, user: &User, course: &Course) -> FieldResult<Enrollment> {
        let conn = &mut context.diesel_pool.get().await?;

        let enrollment = Enrollment {
            id: Uuid::new_v4(),
            user_id: user.id,
            course_id: course.id,
            enrolled_date: chrono::Utc::now(),
        };

        match diesel::insert_into(enrollments::table)
            .values(&enrollment)
            .execute(conn)
            .await
        {
            Ok(_) => Ok(enrollment),
            Err(_) => Err(ErrorCode::CouldNotEnroll.into())
        }
    }
}