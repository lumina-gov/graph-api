mod shared;
use serde_json::json;

#[tokio::test]
async fn can_check_for_active_subscription() -> Result<(), anyhow::Error> {
    let user_email = shared::create_user().await?;
    let token = shared::login_specific(&user_email).await?;

    let res = shared::query(
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
    ).await?;

    assert_eq!(res["errors"], json!(null));
    assert_eq!(res["data"]["me"]["stripe_subscription_info"]["status"], "NONE");
    assert_eq!(res["data"]["me"]["stripe_subscription_info"]["expiry_date"], json!(null));

    Ok(())
}