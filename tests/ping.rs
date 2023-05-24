mod shared;

#[tokio::test]
async fn test_ping() -> Result<(), anyhow::Error> {
    let docker_client = testcontainers::clients::Cli::docker();
    let shared_app = shared::SharedApp::init(&docker_client).await;
    let res = shared_app
        .query(
            "
        query {
            ping
        }
    ",
            &None,
        )
        .await?;

    assert_eq!(res["data"]["ping"], "pong");

    Ok(())
}
