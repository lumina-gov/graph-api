use crate::guards::auth::AuthGuard;
use async_graphql::{Context, Object};

use crate::graphql::types::{question_assessment::QuestionAssessment, user::User};

#[derive(Default)]
pub struct QuestionAssessmentQuery;

#[Object]
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

        unimplemented!()
        // let conn = &mut ctx.data_unchecked::<DieselPool>().get().await?;

        // match question_assessments::table
        //     .filter(question_assessments::user_id.eq(user.id))
        //     .filter(question_assessments::course_slug.eq(course_slug))
        //     .filter(question_assessments::unit_slug.eq(unit_slug))
        //     .filter(question_assessments::question_slug.eq(question_slug))
        //     .first::<Self>(conn)
        //     .await
        //     .optional()
        // {
        //     Ok(assessment) => Ok(assessment),
        //     Err(e) => Err(e.into()),
        // }
    }
}
