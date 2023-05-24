mod shared;

#[tokio::test]
async fn test_user_count() -> Result<(), anyhow::Error> {
    let docker_client = testcontainers::clients::Cli::docker();
    let shared_app = shared::SharedApp::init(&docker_client).await;

    let res = shared_app
        .query(
            "
        query {
            user_count
        }
    ",
            &None,
        )
        .await?;

    assert!(res["data"]["user_count"].is_number());

    Ok(())
}
