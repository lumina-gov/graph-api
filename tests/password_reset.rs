use sea_orm::{ConnectionTrait, Database, EntityTrait};
use serde_json::json;
mod shared;

#[tokio::test]
async fn password_actually_got_reset() -> Result<(), anyhow::Error> {
    let shared_app = shared::SharedApp::init().await;

    let email = shared_app.create_user().await?;

    let _ = shared_app
        .query(
            &format!(
                "
            mutation {{
                reset_password(email:\"{}\")
            }}
    ",
                email
            ),
            &None,
        )
        .await?;

    assert_eq!(response["errors"], json!(null));

    // Check that the password token was added into the database
    let db_url = shared_app.get_db_url();

    let db = Database::connect(&db_url).await.unwrap();
    let db_result = graph_api::schema::password_reset_tokens::Entity::find()
        .one(&db)
        .await
        .unwrap();
    assert!(db_result.is_some(), "should return a token");
    let db_result = db_result.unwrap();

    let response = shared_app
        .query(
            &format!(
                "
        mutation {{
            reset_to_new_password(token_id:\"{}\" new_password:\"{}\")
        }}
",
                db_result.id.simple().to_string(),
                "new_password"
            ),
            &None,
        )
        .await?;
    assert_eq!(response["errors"], json!(null));
    let token = shared_app
        .login_specific_with_password(&email, "new_password")
        .await?;

    assert!(token.is_some(), "should return a token");

    Ok(())
}
