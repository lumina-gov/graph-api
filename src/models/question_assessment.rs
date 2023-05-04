use chrono::serde::ts_milliseconds;
use chrono::{Utc, DateTime};
use diesel::{Identifiable, Queryable, Insertable, ExpressionMethods, Associations, QueryDsl, OptionalExtension};
use diesel_async::RunQueryDsl;
use diesel_derive_enum::DbEnum;
use juniper::{GraphQLObject, GraphQLEnum};
use openai::chat::{ChatCompletionMessage, ChatCompletionMessageRole};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::models::schema::question_assessments;

use crate::graph::context::UniqueContext;

use super::user::User;

const MODEL: &'static str = "gpt-3.5-turbo";

#[derive(Debug, GraphQLEnum, Clone, Copy, Serialize, Deserialize, DbEnum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
#[ExistingTypePath = "crate::models::schema::sql_types::Assessment"]
pub enum Assessment {
    Pass,
    SoftPass,
    Fail,
    Unknown,
}

#[derive(Debug, GraphQLObject, Serialize, Deserialize, Clone, Identifiable, Queryable, Insertable, Associations)]
#[diesel(belongs_to(User))]
#[graphql(rename_all = "none")]
pub struct QuestionAssessment {
    id: Uuid,
    user_id: Uuid,
    course_slug: String,
    unit_slug: String,
    question_slug: String,
    answer: String,
    assessment: Assessment,
    feedback: String,
    #[serde(with = "ts_milliseconds")]
    updated_at: DateTime<Utc>,
}

impl QuestionAssessment {
    pub async fn create_assessment(
        context: &UniqueContext,
        user: &User,
        course_slug: String,
        unit_slug: String,
        question_slug: String,
        question: String,
        answer: String,
        question_context: Option<String>,
    ) -> Result<Self, anyhow::Error> {
        let message = ChatCompletionMessage {
            content: format!(r#"
Assess the user's response, and provide feedback and corrections if necessary.

type HumanString = string
type Response = {{
    feedback: HumanString
    assessment: Assessment
}}
type Assessment = "PASS" | "SOFT_PASS" | "FAIL" | "UNKNOWN"

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
                question,
                match question_context {
                    Some(question_context) => format!("Context\n{}", question_context),
                    None => String::new(),
                },
                answer,
            ),
            name: Some(user.first_name.clone()),
            role: ChatCompletionMessageRole::User,
        };

        let response = openai::chat::ChatCompletion::builder(MODEL, [message])
            .create()
            .await??;

        let json_string: String = format!(r#"{{ "feedback": "{}"#, response.choices[0].message.content);

        #[derive(Debug, Deserialize)]
        struct PartialAssessment {
            feedback: String,
            assessment: Assessment,
        }

        let partial_assessment: PartialAssessment = serde_json::from_str(&json_string).map_err(|_| anyhow::anyhow!("Failed to serialise AI response, please try again"))?;

        let assessment = QuestionAssessment {
            id: Uuid::new_v4(),
            user_id: user.id,
            course_slug,
            unit_slug,
            question_slug,
            answer,
            assessment: partial_assessment.assessment,
            feedback: partial_assessment.feedback,
            updated_at: Utc::now(),
        };

        let conn = &mut context.diesel_pool.get().await?;

        match diesel::insert_into(question_assessments::table)
            .values(&assessment)
            .on_conflict((
                question_assessments::user_id,
                question_assessments::course_slug,
                question_assessments::unit_slug,
                question_assessments::question_slug
            ))
            .do_update()
            .set((
                question_assessments::feedback.eq(&assessment.feedback),
                question_assessments::answer.eq(&assessment.answer),
                question_assessments::assessment.eq(&assessment.assessment),
                question_assessments::updated_at.eq(Utc::now()),
            ))
            .get_result(conn)
            .await
        {
            Ok(assessment) => Ok(assessment),
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) async fn get_question_assessment(
        context: &UniqueContext,
        user: &User,
        course_slug: String,
        unit_slug: String,
        question_slug: String,
    ) -> Result<Option<Self>, anyhow::Error> {
        let conn = &mut context.diesel_pool.get().await?;

        match question_assessments::table
            .filter(question_assessments::user_id.eq(user.id))
            .filter(question_assessments::course_slug.eq(course_slug))
            .filter(question_assessments::unit_slug.eq(unit_slug))
            .filter(question_assessments::question_slug.eq(question_slug))
            .first::<Self>(conn)
            .await
            .optional()
        {
            Ok(assessment) => Ok(assessment),
            Err(e) => Err(e.into()),
        }
    }

}