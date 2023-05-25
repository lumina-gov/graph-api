mod shared;

#[tokio::test]
async fn test_ping() -> Result<(), anyhow::Error> {
    let shared_app = shared::SharedApp::init().await;
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
