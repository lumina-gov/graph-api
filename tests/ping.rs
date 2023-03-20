use graph_api::App;
use lambda_http::Error;

#[tokio::test]
async fn test_ping() -> Result<(), anyhow::Error> {
    let res = graph_api::test_utils::query("
        query {
            ping
        }
    ", None).await?;

    dbg!(&res);

    assert_eq!(res["data"]["ping"], "pong");

    Ok(())
}