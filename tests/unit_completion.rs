use serde_json::json;


mod shared;

pub async fn set_unit_progress(
    course_slug: &str,
    unit_slug: &str,
    status: &str,
    token: &Option<String>
) -> Result<serde_json::Value, anyhow::Error> {
    let query = format!(
        r#"
        mutation {{
            set_unit_progress(course_slug: "{}", unit_slug: "{}", status: {}) {{
                id
                status
                user_id
                unit_slug
                course_slug
                updated_at
            }}
        }}
    "#,
        course_slug, unit_slug, status
    );

    shared::query(&query, &token).await
}


async fn last_updated_unit(token: &Option<String>) -> Result<serde_json::Value, anyhow::Error> {
    shared::query(
        r#"
        query {
            last_updated_unit {
                id
                status
                user_id
                unit_slug
                course_slug
                updated_at
            }
        }
    "#,
        &token,
    ).await
}

#[tokio::test]
async fn mark_unit_as_completed() -> Result<(), anyhow::Error> {
    let user_email = shared::create_user().await?;
    let token = shared::login_specific(&user_email).await?;

    let res_1 = set_unit_progress("foo", "bar", "IN_PROGRESS", &token).await?;

    assert_eq!(res_1["errors"], json!(null));
    assert_eq!(res_1["data"]["set_unit_progress"]["status"], "IN_PROGRESS");
    assert_eq!(res_1["data"]["set_unit_progress"]["unit_slug"], "bar");
    assert_eq!(res_1["data"]["set_unit_progress"]["course_slug"], "foo");

    // set it to COMPLETED, and validate that the id is the same
    let res_2 = set_unit_progress("foo", "bar", "COMPLETED", &token).await?;

    assert_eq!(res_2["errors"], json!(null));
    assert_eq!(res_2["data"]["set_unit_progress"]["status"], "COMPLETED");
    assert_eq!(res_2["data"]["set_unit_progress"]["unit_slug"], "bar");
    assert_eq!(res_2["data"]["set_unit_progress"]["course_slug"], "foo");
    assert_eq!(res_2["data"]["set_unit_progress"]["id"], res_1["data"]["set_unit_progress"]["id"]);

    Ok(())
}

#[tokio::test]
async fn can_get_course_progress() -> Result<(), anyhow::Error> {
    let user_email = shared::create_user().await?;
    let token = shared::login_specific(&user_email).await?;

    set_unit_progress("foo", "bar", "IN_PROGRESS", &token).await?;

    let res_2 = shared::query(
        r#"
        query {
            course_progress(course_slug: "foo") {
                id
                status
                user_id
                unit_slug
                course_slug
                updated_at
            }
        }
    "#,
        &token,
    ).await?;

    assert_eq!(res_2["errors"], json!(null));
    assert_eq!(res_2["data"]["course_progress"][0]["status"], "IN_PROGRESS");
    assert_eq!(res_2["data"]["course_progress"][0]["unit_slug"], "bar");

    Ok(())
}

#[tokio::test]
async fn can_get_all_course_progress() -> Result<(), anyhow::Error> {
    let user_email = shared::create_user().await?;
    let token = shared::login_specific(&user_email).await?;

    set_unit_progress("foo", "bar", "IN_PROGRESS", &token).await?;
    set_unit_progress("xyz", "bar", "IN_PROGRESS", &token).await?;

    let res_2 = shared::query(
        r#"
        query {
            all_course_progress {
                id
                status
                user_id
                unit_slug
                course_slug
                updated_at
            }
        }
    "#,
        &token,
    ).await?;

    assert_eq!(res_2["errors"], json!(null));

    assert_eq!(res_2["data"]["all_course_progress"][0][0]["status"], "IN_PROGRESS");
    assert_eq!(res_2["data"]["all_course_progress"][0][0]["unit_slug"], "bar");

    assert_eq!(res_2["data"]["all_course_progress"][1][0]["status"], "IN_PROGRESS");
    assert_eq!(res_2["data"]["all_course_progress"][1][0]["unit_slug"], "bar");

    // order isn't guaranteed, so check that both course_slugs are present
    let mut found_foo = false;
    let mut found_xyz = false;

    for course_progress in res_2["data"]["all_course_progress"].as_array().unwrap() {
        if course_progress[0]["course_slug"] == "foo" {
            found_foo = true;
        }

        if course_progress[0]["course_slug"] == "xyz" {
            found_xyz = true;
        }
    }

    assert!(found_foo);
    assert!(found_xyz);

    Ok(())
}


#[tokio::test]
async fn can_get_last_updated_unit() -> Result<(), anyhow::Error> {
    let user_email = shared::create_user().await?;
    let token = shared::login_specific(&user_email).await?;

    // User's last updated unit should be null if they haven't completed any units
    let res_1 = last_updated_unit(&token).await?;

    assert_eq!(res_1["errors"], json!(null));
    assert_eq!(res_1["data"]["last_updated_unit"], json!(null));

    // Set a unit to COMPLETED, and validate that the last_updated_unit is set
    set_unit_progress("foo", "bar", "COMPLETED", &token).await?;

    let res_2 = last_updated_unit(&token).await?;

    assert_eq!(res_2["errors"], json!(null));
    assert_eq!(res_2["data"]["last_updated_unit"]["status"], "COMPLETED");
    assert_eq!(res_2["data"]["last_updated_unit"]["unit_slug"], "bar");
    assert_eq!(res_2["data"]["last_updated_unit"]["course_slug"], "foo");

    // Set a unit to IN_PROGRESS, and validate that this is the new last_updated_unit
    set_unit_progress("foo", "xyz", "IN_PROGRESS", &token).await?;

    let res_3 = last_updated_unit(&token).await?;

    assert_eq!(res_3["errors"], json!(null));
    assert_eq!(res_3["data"]["last_updated_unit"]["status"], "IN_PROGRESS");
    assert_eq!(res_3["data"]["last_updated_unit"]["unit_slug"], "xyz");
    assert_eq!(res_3["data"]["last_updated_unit"]["course_slug"], "foo");

    Ok(())
}
