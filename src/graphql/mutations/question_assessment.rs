use async_graphql::{Context, Object};
use chrono::Utc;
use openai::chat::{
    ChatCompletionFunctionDefinition, ChatCompletionMessage, ChatCompletionMessageRole,
};
use sea_orm::{sea_query::OnConflict, DatabaseConnection, EntityTrait};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    error::new_err,
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

const MODEL: &str = "gpt-3.5-turbo-0613";

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
    ) -> async_graphql::Result<QuestionAssessment> {
        let user = ctx.data_unchecked::<User>();
        let conn = ctx.data_unchecked::<DatabaseConnection>();

        let messages = [
            ChatCompletionMessage {
                content: Some(format!(
                    r#"
- Assess the user's response/answer, and provide feedback and corrections in the function call feedback parameter.
- If the answer is a SOFT_PASS or FAIL, explain how the answer can be improved.
- Be strict and fail if the answer is not sufficient.
- Use 'UNKNOWN' if the user did not answer the question.
- Feedback can contain any markdown formatting (e.g. **bold**, *italics*, `code`, etc)

Course Slug: {}
Unit Slug: {}

Question
{}

{}"#,
                    course_slug,
                    unit_slug,
                    question,
                    match question_context {
                        Some(question_context) =>
                            format!("Additional Context\n{}", question_context),
                        None => String::new(),
                    },
                )),
                name: None,
                role: ChatCompletionMessageRole::System,
                function_call: None,
            },
            ChatCompletionMessage {
                content: Some(answer.clone()),
                name: None,
                role: ChatCompletionMessageRole::User,
                function_call: None,
            },
        ];

        let response = openai::chat::ChatCompletion::builder(MODEL, messages)
            .functions([
                ChatCompletionFunctionDefinition {
                    name: "ai_assessment".into(),
                    description: Some("Write an AI teacher assessment of the user's answer to a given question".into()),
                    parameters: Some(serde_json::json!({
                        "type": "object",
                        "properties": {
                            "feedback": {
                                "type": "string",
                                "description": "AI assessment of the the user's response\nprovide feedback and corrections as markdown string"
                            },
                            "assessment": {
                                "type": "string",
                                "enum": ["PASS", "SOFT_PASS", "FAIL", "UNKNOWN"],
                                "description": "Did the user accurately answer the question?"
                            }
                        },
                        "required": ["assessment", "feedback"]
                    })),
                },
            ])
            .user(slug::slugify(&user.first_name))
            .create()
            .await?;

        let reply = &response.choices[0].message;

        let function_call = reply.function_call.as_ref().ok_or_else(|| {
            new_err(
                "MISSING_FUNCTION_CALL",
                "OpenAI did not return a function call in the response",
            )
        })?;

        #[derive(Deserialize)]
        struct PartialAssessment {
            feedback: String,
            assessment: Assessment,
        }

        let partial_assessment =
            serde_json::from_str::<PartialAssessment>(&function_call.arguments).map_err(|_| {
                new_err(
                    "INVALID_ARGUMENTS",
                    "Failed to parse function call parameters from OpenAI",
                )
            })?;

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
