use async_graphql::{Context, Object};
use chrono::Utc;
use openai::chat::{ChatCompletionMessage, ChatCompletionMessageRole};
use sea_orm::{sea_query::OnConflict, DatabaseConnection, EntityTrait};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    error::APIError,
    graphql::types::{
        question_assessment::{
            QuestionAssessment, QuestionAssessmentActiveModel, QuestionAssessmentColumn,
            QuestionAssessmentEntity,
        },
        user::User,
    },
    guards::auth::AuthGuard,
    schema::sea_orm_active_enums::Assessment,
};

const MODEL: &str = "gpt-3.5-turbo";

#[derive(Default)]
pub struct QuestionAssessmentMutation;

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl QuestionAssessmentMutation {
    #[graphql(guard = "AuthGuard")]
    pub async fn question_assessment(
        &self,
        ctx: &Context<'_>,
        course_slug: String,
        unit_slug: String,
        question_slug: String,
        question: String,
        answer: String,
        question_context: Option<String>,
    ) -> Result<QuestionAssessment, anyhow::Error> {
        let user = ctx.data_unchecked::<User>();
        let conn = ctx.data_unchecked::<DatabaseConnection>();

        let message = ChatCompletionMessage {
            content: format!(
                r#"
Assess the user's response, and provide feedback and corrections if necessary.
If the answer is a SOFT_PASS or FAIL, explain how the answer can be improved.

type HumanString = string
type Response = {{
    feedback: HumanString
    assessment: Assessment
}}
type Assessment = "PASS" | "SOFT_PASS" | "FAIL" | "UNKNOWN"

Course Slug: "{}"
Unit Slug: "{}"
Question:
{}
{}
User Answer:
{}
<END USER ANSWER>

Respond in Pure JSON
---
{{
    "feedback": ""#,
                course_slug,
                unit_slug,
                question,
                match question_context {
                    Some(question_context) => format!("Context\n{}", question_context),
                    None => String::new(),
                },
                answer,
            ),
            name: Some(slug::slugify(&user.first_name)),
            role: ChatCompletionMessageRole::User,
        };

        let response = openai::chat::ChatCompletion::builder(MODEL, [message])
            .create()
            .await??;

        let content = response.choices[0].message.content.clone();

        let json_string: String = format!(r#"{{ "feedback": "{}"#, content);

        #[derive(Debug, Deserialize)]
        struct PartialAssessment {
            feedback: String,
            assessment: Assessment,
        }

        let partial_assessment: PartialAssessment = match serde_json::from_str(&json_string) {
            Ok(partial_assessment) => partial_assessment,
            Err(err) => Err(APIError::new_with_detail(
                "FAILED_DESERIALIZATION",
                "Failed to deserialize AI response. Please try again.",
                &err.to_string(),
            ))?,
        };

        let assessment: QuestionAssessmentActiveModel = QuestionAssessment {
            id: Uuid::new_v4(),
            user_id: user.id,
            course_slug,
            unit_slug,
            question_slug,
            answer,
            assessment: partial_assessment.assessment,
            feedback: partial_assessment.feedback,
            updated_at: Utc::now(),
        }
        .into();

        Ok(QuestionAssessmentEntity::insert(assessment)
            .on_conflict(
                OnConflict::columns([
                    QuestionAssessmentColumn::UserId,
                    QuestionAssessmentColumn::CourseSlug,
                    QuestionAssessmentColumn::UnitSlug,
                    QuestionAssessmentColumn::QuestionSlug,
                ])
                .update_columns([
                    QuestionAssessmentColumn::Answer,
                    QuestionAssessmentColumn::Assessment,
                    QuestionAssessmentColumn::Feedback,
                    QuestionAssessmentColumn::UpdatedAt,
                ])
                .to_owned(),
            )
            .exec_with_returning(conn)
            .await?)
    }
}
