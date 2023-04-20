mod shared;

#[tokio::test]
async fn test_ping() -> Result<(), anyhow::Error> {
    let res = shared::query("
        query {
            ping
        }
    ", &None).await?;

    dbg!(&res);

    assert_eq!(res["data"]["ping"], "pong");

    Ok(())
}