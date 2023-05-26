use serde_json::json;

mod shared;

#[tokio::test]
async fn can_query_auth_app() -> Result<(), anyhow::Error> {
    let shared_app = shared::SharedApp::init().await;

    let response = shared_app
        .query(
            r#"
        query {
            auth_app(slug: "lumina-university") {
                name
                description
                created
                official
                redirect_hostnames
                scopes
            }
        }
    "#,
            &None,
        )
        .await?;

    assert_eq!(response["errors"], json!(null));
    assert_eq!(
        response["data"]["auth_app"]["name"],
        json!("Lumina University")
    );
    assert_eq!(
        response["data"]["auth_app"]["description"],
        json!("The next generation of education")
    );
    assert_eq!(
        response["data"]["auth_app"]["created"],
        json!("2023-05-25T00:00:00+00:00")
    );
    assert_eq!(response["data"]["auth_app"]["official"], json!(true));
    assert_eq!(
        response["data"]["auth_app"]["redirect_hostnames"],
        json!(["luminauniversity.earth", "localhost"])
    );
    assert_eq!(
        response["data"]["auth_app"]["scopes"],
        json!(["profile:read", "billing", "education"])
    );

    Ok(())
}
