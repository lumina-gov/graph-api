use serde_json::json;

mod shared;

#[tokio::test]
async fn can_get_me() -> Result<(), anyhow::Error> {
    let docker_client = testcontainers::clients::Cli::docker();
    let shared_app = shared::SharedApp::init(&docker_client).await;

    let email = shared_app.create_user().await?;
    let token = shared_app.login_specific(&email).await?;

    let response = shared_app
        .query(
            r#"
        query {
            me {
                id
                email
                first_name
                last_name,
                stripe_subscription_info {
                    status
                    expiry_date
                }
            }
        }
    "#,
            &token,
        )
        .await?;

    assert_eq!(response["errors"], json!(null));
    assert_eq!(response["data"]["me"]["email"], json!(email));
    assert_eq!(response["data"]["me"]["first_name"], json!("John"));
    assert_eq!(response["data"]["me"]["last_name"], json!("Doe"));
    assert_eq!(
        response["data"]["me"]["stripe_subscription_info"]["status"],
        json!("NONE")
    );

    Ok(())
}
