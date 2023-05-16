mod shared;

#[tokio::test]
async fn test_user_count() -> Result<(), anyhow::Error> {
    let res = shared::query(
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
