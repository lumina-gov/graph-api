use serde_json::json;

mod shared;

#[tokio::test]
async fn can_query_citizenship_status() -> Result<(), anyhow::Error> {
    let shared_app = shared::SharedApp::init().await;

    let email = shared_app.create_user().await?;
    let token = shared_app.login_specific(&email).await?;

    let response = shared_app
        .query(
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

#[tokio::test]
async fn can_create_ctiizenship_application() -> Result<(), anyhow::Error> {
    let shared_app = shared::SharedApp::init().await;

    let email = shared_app.create_user().await?;
    let token = shared_app.login_specific(&email).await?;

    let response = shared_app
        .query(
            r#"
        mutation {
            create_citizenship_application (
                date_of_birth: 1,
                sex: "MALE",
                first_name: "John",
                last_name: "Doe",
                skills: ["skill1", "skill2"],
                occupations: ["occupation1", "occupation2"],
                country_of_citizenship: ["country1", "country2"],
                country_of_birth: "country",
                country_of_residence: "country",
                ethnic_groups: ["ethnic1", "ethnic2"],
            )
        }
    "#,
            &token,
        )
        .await?;

    assert_eq!(response["errors"], json!(null));

    Ok(())
}
