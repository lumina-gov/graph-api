use serde_json::json;

mod shared;

#[tokio::test]
async fn can_query_citizenship_status() -> Result<(), anyhow::Error> {
    let email = shared::create_user().await?;
    let token = shared::login_specific(&email).await?;

    let response = shared::query(
        r#"
        query {
            me {
                citizenship_status
            }
        }
    "#,
        &token,
    )
    .await?;

    assert_eq!(response["errors"], json!(null));
    assert_eq!(response["data"]["me"]["citizenship_status"], json!(null));

    Ok(())
}
