use serde_json::json;

mod shared;

#[tokio::test]
async fn request_without_scopes_fails() -> Result<(), anyhow::Error> {
    let shared_app = shared::SharedApp::init().await;
    let email = shared_app.create_user().await?;
    let token = shared_app
        .login_specific_with_scopes(&email, vec![])
        .await?;

    let response = shared_app
        .query(
            r#"
        query {
            me {
                first_name
            }
        }
    "#,
            &token,
        )
        .await
        .unwrap();

    assert_eq!(
        response["errors"][0]["extensions"]["code"],
        json!("UNAUTHORIZED")
    );

    Ok(())
}

#[tokio::test]
async fn request_with_scopes_succeeds() -> Result<(), anyhow::Error> {
    let shared_app = shared::SharedApp::init().await;
    let email = shared_app.create_user().await?;
    let token = shared_app
        .login_specific_with_scopes(&email, vec!["profile:read:name"])
        .await?;

    let response = shared_app
        .query(
            r#"
        query {
            me {
                first_name
            }
        }
    "#,
            &token,
        )
        .await
        .unwrap();

    assert_eq!(response["errors"], json!(null));
    assert_eq!(response["data"]["me"]["first_name"], json!("John"));

    Ok(())
}
