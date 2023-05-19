use serde_json::json;

mod shared;

#[tokio::test]
async fn can_do_question_assessment() -> Result<(), anyhow::Error> {
    let email = shared::create_user().await?;
    let token = shared::login_specific(&email).await?;

    let response = create_question_assessment(
        "test-course",
        "test-unit",
        "test-question",
        "What is 1+1?",
        "2",
        "mathematics",
        &token,
    )
    .await?;

    assert_eq!(response["errors"], json!(null));

    assert!(response["data"]["question_assessment"]["feedback"].is_string());
    assert!(response["data"]["question_assessment"]["assessment"].is_string());

    let response =
        get_question_assessment("test-course", "test-unit", "test-question", &token).await?;

    assert_eq!(response["errors"], json!(null));

    assert_eq!(response["data"]["question_assessment"]["answer"], "2");

    Ok(())
}

async fn create_question_assessment(
    course_slug: &str,
    unit_slug: &str,
    question_slug: &str,
    question: &str,
    answer: &str,
    context: &str,
    token: &Option<String>,
) -> Result<serde_json::Value, anyhow::Error> {
    let query = format!(
        r#"
        mutation {{
            question_assessment(course_slug: "{}", unit_slug: "{}", question_slug: "{}", question: "{}", answer: "{}", question_context: "{}") {{
                unit_slug
                question_slug
                answer
                feedback
                assessment
                user_id
            }}
        }}
    "#,
        course_slug, unit_slug, question_slug, question, answer, context,
    );

    shared::query(&query, token).await
}

async fn get_question_assessment(
    course_slug: &str,
    unit_slug: &str,
    question_slug: &str,
    token: &Option<String>,
) -> Result<serde_json::Value, anyhow::Error> {
    let query = format!(
        r#"
        query {{
            question_assessment(course_slug: "{}", unit_slug: "{}", question_slug: "{}") {{
                unit_slug
                question_slug
                answer
                feedback
                assessment
                user_id
            }}
        }}
    "#,
        course_slug, unit_slug, question_slug,
    );

    shared::query(&query, token).await
}
