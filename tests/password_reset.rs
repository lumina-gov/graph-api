use chrono::{Duration, Utc};
use sea_orm::{
    ColumnTrait, ConnectionTrait, Database, EntityTrait, IntoActiveModel, QueryFilter, QuerySelect,
};
use serde_json::json;
mod shared;

#[tokio::test]
async fn password_actually_got_reset() -> Result<(), anyhow::Error> {
    let shared_app = shared::SharedApp::init().await;

    let email = shared_app.create_user().await?;

    let response = shared_app
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

#[tokio::test]
async fn password_does_not_reset_stale_token() -> Result<(), anyhow::Error> {
    let shared_app = shared::SharedApp::init().await;
    let email = shared_app.create_user().await?;

    let db_url = shared_app.get_db_url();
    let db = Database::connect(&db_url).await.unwrap();

    let created_user = graph_api::schema::users::Entity::find()
        .filter(graph_api::schema::users::Column::Email.eq(email))
        .one(&db)
        .await?
        .unwrap();

    let token_id = uuid::Uuid::new_v4();
    let token = graph_api::schema::password_reset_tokens::Model {
        user_id: created_user.id,
        id: token_id,
        expires_at: Utc::now() - Duration::minutes(5),
    };

    graph_api::schema::password_reset_tokens::Entity::insert(token.into_active_model())
        .exec_without_returning(&db)
        .await?;

    let response = shared_app
        .query(
            &format!(
                "
        mutation {{
            reset_to_new_password(token_id:\"{}\" new_password:\"{}\")
        }}
",
                token_id.simple().to_string(),
                "new_password"
            ),
            &None,
        )
        .await?;

    assert_eq!(response["errors"][0]["extensions"]["code"], "TOKEN_EXPIRED");

    Ok(())
}
