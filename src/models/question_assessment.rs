use juniper::{GraphQLObject, GraphQLEnum};
use openai::chat::{ChatCompletionMessage, ChatCompletionMessageRole, ChatCompletion, ChatCompletionBuilder};
use serde::{Serialize, Deserialize};

use crate::graph::context::UniqueContext;

use super::user::User;

const MODEL: &'static str = "gpt-3.5-turbo";

#[derive(Debug, GraphQLEnum, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Assessment {
    Pass,
    SoftPass,
    Fail,
    Unknown,
}

#[derive(Debug, GraphQLObject, Serialize, Deserialize, Clone)]
#[graphql(rename_all = "none")]
pub struct QuestionAssessment {
    feedback: String,
    assessment: Assessment,
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
User Answer:
{}
<END USER ANSWER>

Respond in Pure JSON
---
{{
    "feedback": ""#, question, answer),
            name: Some(user.first_name.clone()),
            role: ChatCompletionMessageRole::User,
        };

        let response = openai::chat::ChatCompletion::builder(MODEL, [message])
            .create()
            .await??;

        let json = format!(r#"{{ "feedback": "{}"#, response.choices[0].message.content);

        println!("{}", json);
        dbg!(serde_json::from_str::<serde_json::Value>(&json)?);

        Ok(serde_json::from_str(&json).map_err(|_| anyhow::anyhow!("Failed to serialise AI response, please try again"))?)
    }
}