use serde_json::json;

mod shared;

#[tokio::test]
async fn can_do_question_assessment() -> Result<(), anyhow::Error> {
    let email = shared::create_user().await?;
    let token = shared::login_specific(&email).await?;

    let query = format!(
        r#"
        mutation {{
            question_assessment(course_slug: "{}", unit_slug: "{}", question_slug: "{}", question: "{}", answer: "{}") {{
                feedback
                assessment
            }}
        }}
    "#,
        "test-course",
        "test-unit",
        "test-question",
        "What is 1+1?",
        "2",
    );

    let response = shared::query(&query, &token).await?;

    assert_eq!(response["errors"], json!(null));

    assert!(response["data"]["question_assessment"]["feedback"].is_string());
    assert!(response["data"]["question_assessment"]["assessment"].is_string());

    Ok(())
}
