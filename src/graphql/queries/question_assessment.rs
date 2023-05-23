use crate::{
    graphql::types::question_assessment::{QuestionAssessmentColumn, QuestionAssessmentEntity},
    guards::auth::AuthGuard,
};
use async_graphql::{Context, Object};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::graphql::types::{question_assessment::QuestionAssessment, user::User};

#[derive(Default)]
pub struct QuestionAssessmentQuery;

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl QuestionAssessmentQuery {
    #[graphql(guard = "AuthGuard")]
    async fn question_assessment(
        &self,
        ctx: &Context<'_>,
        course_slug: String,
        unit_slug: String,
        question_slug: String,
    ) -> Result<Option<QuestionAssessment>, anyhow::Error> {
        let user = ctx.data_unchecked::<User>();
        let conn = ctx.data_unchecked::<DatabaseConnection>();

        Ok(QuestionAssessmentEntity::find()
            .filter(QuestionAssessmentColumn::UserId.eq(user.id))
            .filter(QuestionAssessmentColumn::CourseSlug.eq(course_slug))
            .filter(QuestionAssessmentColumn::UnitSlug.eq(unit_slug))
            .filter(QuestionAssessmentColumn::QuestionSlug.eq(question_slug))
            .one(conn)
            .await?)
    }
}
