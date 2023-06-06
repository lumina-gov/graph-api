use serde_json::json;

mod shared;

#[tokio::test]
async fn can_get_me() -> Result<(), anyhow::Error> {
    let shared_app = shared::SharedApp::init().await;

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

#[tokio::test]
async fn duplicate_user_fails() -> Result<(), anyhow::Error> {
    let shared_app = shared::SharedApp::init().await;
    let email = "hello@123.com";

    let res = shared_app
        .query(
            format!(
                r#"
            mutation {{
                create_user(
                    email: "{}",
                    password: "password",
                    first_name: "John",
                    last_name: "Doe",
                    calling_code: "1",
                    country_code: "US",
                    phone_number: "555-555-5555"
                )
            }}
        "#,
                email
            )
            .as_str(),
            &None,
        )
        .await?;

    assert_eq!(res["errors"], json!(null));

    let res = shared_app
        .query(
            format!(
                r#"
            mutation {{
                create_user(
                    email: "{}",
                    password: "password",
                    first_name: "John",
                    last_name: "Doe",
                    calling_code: "1",
                    country_code: "US",
                    phone_number: "555-555-5555"
                )
            }}
        "#,
                email
            )
            .as_str(),
            &None,
        )
        .await?;

    assert_eq!(
        res["errors"][0]["extensions"]["code"],
        json!("USER_ALREADY_EXISTS")
    );

    Ok(())
}
