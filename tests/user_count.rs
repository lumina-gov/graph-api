mod shared;

#[tokio::test]
async fn test_user_count() -> Result<(), anyhow::Error> {
    let shared_app = shared::SharedApp::init().await;

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

#[tokio::test]
async fn can_get_user_count_by_interval() -> Result<(), anyhow::Error> {
    let shared_app = shared::SharedApp::init().await;

    let res = shared_app
        .query(
            "
        query {
            user_count_by_interval(interval: 1, count: 12)
        }
    ",
            &None,
        )
        .await?;

    assert!(res["data"]["user_count_by_interval"].is_array());

    Ok(())
}
