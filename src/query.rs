use async_graphql::{Context, MergedObject, Object};
use zxcvbn::time_estimates::CrackTimeSeconds;

use crate::{
    guards::auth::AuthGuard,
    misc::CrackSeconds,
    types::{
        question_assessment::QuestionAssessment,
        unit_progress::UnitProgress,
        user::{User, UserQuery},
    },
};

#[derive(MergedObject, Default)]
pub(crate) struct Query(BaseQuery, UserQuery);

#[derive(Default)]
struct BaseQuery;

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl BaseQuery {
    async fn ping(&self) -> String {
        "pong".to_string()
    }

    /// Returns the crack time of a password
    /// Used for password strength estimation
    /// On the frontend
    async fn crack_time(&self, password: String) -> CrackSeconds {
        let guesses = match zxcvbn::zxcvbn(&password, &[]) {
            Ok(entropy) => entropy.guesses(),
            Err(_) => 0,
        } as f64;

        CrackSeconds {
            guesses,
            seconds: guesses as f64 / 100_000.0,
            string: CrackTimeSeconds::Float(guesses as f64 / 100_000.0).to_string(),
        }
    }

    #[graphql(guard = "AuthGuard")]
    async fn course_progress(
        &self,
        ctx: &Context<'_>,
        course_slug: String,
    ) -> Result<Vec<UnitProgress>, anyhow::Error> {
        let user = ctx.data_unchecked::<User>();

        let progress = UnitProgress::course_progress(ctx, user, course_slug).await?;

        Ok(progress)
    }

    #[graphql(guard = "AuthGuard")]
    async fn all_course_progress(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<Vec<UnitProgress>>, anyhow::Error> {
        let user = ctx.data_unchecked::<User>();
        Ok(UnitProgress::all_course_progress(ctx, user).await?)
    }

    #[graphql(guard = "AuthGuard")]
    async fn last_updated_unit(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<UnitProgress>, anyhow::Error> {
        let user = ctx.data_unchecked::<User>();

        Ok(UnitProgress::last_updated_unit(ctx, user).await?)
    }

    #[graphql(guard = "AuthGuard")]
    async fn question_assessment(
        &self,
        ctx: &Context<'_>,
        course_slug: String,
        unit_slug: String,
        question_slug: String,
    ) -> Result<Option<QuestionAssessment>, anyhow::Error> {
        let user = ctx.data_unchecked::<User>();

        Ok(QuestionAssessment::get_question_assessment(
            ctx,
            user,
            course_slug,
            unit_slug,
            question_slug,
        )
        .await?)
    }
}
