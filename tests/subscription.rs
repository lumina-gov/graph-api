mod shared;
use serde_json::json;

#[tokio::test]
async fn can_check_for_active_subscription() -> Result<(), anyhow::Error> {
    let docker_client = testcontainers::clients::Cli::docker();
    let shared_app = shared::SharedApp::init(&docker_client).await;

    let user_email = shared_app.create_user().await?;
    let token = shared_app.login_specific(&user_email).await?;

    let res = shared_app
        .query(
            r#"
        query {
            me {
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

    assert_eq!(res["errors"], json!(null));
    assert_eq!(
        res["data"]["me"]["stripe_subscription_info"]["status"],
        "NONE"
    );
    assert_eq!(
        res["data"]["me"]["stripe_subscription_info"]["expiry_date"],
        json!(null)
    );

    Ok(())
}

#[tokio::test]
async fn fails() -> Result<(), anyhow::Error> {
    let docker_client = testcontainers::clients::Cli::docker();
    let shared_app = shared::SharedApp::init(&docker_client).await;

    let res = shared_app
        .query(
            r#"
        query {
            all_course_progress {
                course_slug
            }
        }
    "#,
            &None,
        )
        .await?;

    assert_eq!(res["errors"][0]["extensions"]["code"], "UNAUTHENTICATED",);

    Ok(())
}
