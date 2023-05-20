use async_graphql::Context;

use crate::schema::question_assessments::QuestionAssessment;

use super::user::User;

const MODEL: &str = "gpt-3.5-turbo";

// #[derive(Debug, Enum, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, DbEnum)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// #[DbValueStyle = "SCREAMING_SNAKE_CASE"]
// #[ExistingTypePath = "crate::db_schema::sql_types::Assessment"]
// pub enum Assessment {
//     Pass,
//     SoftPass,
//     Fail,
//     Unknown,
// }

impl QuestionAssessment {
    pub async fn create_assessment(
        ctx: &Context<'_>,
        user: &User,
        course_slug: String,
        unit_slug: String,
        question_slug: String,
        question: String,
        answer: String,
        question_context: Option<String>,
    ) -> Result<Self, anyhow::Error> {
        unimplemented!()
        //         let message = ChatCompletionMessage {
        //             content: format!(
        //                 r#"
        // Assess the user's response, and provide feedback and corrections if necessary.
        // If the answer is a SOFT_PASS or FAIL, explain how the answer can be improved.

        // type HumanString = string
        // type Response = {{
        //     feedback: HumanString
        //     assessment: Assessment
        // }}
        // type Assessment = "PASS" | "SOFT_PASS" | "FAIL" | "UNKNOWN"

        // Course Slug: "{}"
        // Unit Slug: "{}"
        // Question:
        // {}
        // {}
        // User Answer:
        // {}
        // <END USER ANSWER>

        // Respond in Pure JSON
        // ---
        // {{
        //     "feedback": ""#,
        //                 course_slug,
        //                 unit_slug,
        //                 question,
        //                 match question_context {
        //                     Some(question_context) => format!("Context\n{}", question_context),
        //                     None => String::new(),
        //                 },
        //                 answer,
        //             ),
        //             name: Some(slug::slugify(&user.first_name)),
        //             role: ChatCompletionMessageRole::User,
        //         };

        //         let response = openai::chat::ChatCompletion::builder(MODEL, [message])
        //             .create()
        //             .await??;

        //         let content = response.choices[0].message.content.clone();

        //         let json_string: String = format!(r#"{{ "feedback": "{}"#, content);

        //         #[derive(Debug, Deserialize)]
        //         struct PartialAssessment {
        //             feedback: String,
        //             assessment: Assessment,
        //         }

        //         let partial_assessment: PartialAssessment =
        //             serde_json::from_str(&json_string).map_err(|_| {
        //                 anyhow::anyhow!(
        //                     "Failed to serialise AI response, please try again. AI Response {}",
        //                     content
        //                 )
        //             })?;

        //         let assessment = QuestionAssessment {
        //             id: Uuid::new_v4(),
        //             user_id: user.id,
        //             course_slug,
        //             unit_slug,
        //             question_slug,
        //             answer,
        //             assessment: partial_assessment.assessment,
        //             feedback: partial_assessment.feedback,
        //             updated_at: Utc::now(),
        //         };

        //         let conn = &mut ctx.data_unchecked::<DieselPool>().get().await?;

        //         match diesel::insert_into(question_assessments::table)
        //             .values(&assessment)
        //             .on_conflict((
        //                 question_assessments::user_id,
        //                 question_assessments::course_slug,
        //                 question_assessments::unit_slug,
        //                 question_assessments::question_slug,
        //             ))
        //             .do_update()
        //             .set((
        //                 question_assessments::feedback.eq(&assessment.feedback),
        //                 question_assessments::answer.eq(&assessment.answer),
        //                 question_assessments::assessment.eq(&assessment.assessment),
        //                 question_assessments::updated_at.eq(Utc::now()),
        //             ))
        //             .get_result(conn)
        //             .await
        //         {
        //             Ok(assessment) => Ok(assessment),
        //             Err(e) => Err(e.into()),
        //         }
    }

    pub(crate) async fn get_question_assessment(
        ctx: &Context<'_>,
        user: &User,
        course_slug: String,
        unit_slug: String,
        question_slug: String,
    ) -> Result<Option<Self>, anyhow::Error> {
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
