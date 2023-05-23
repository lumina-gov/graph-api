use lambda_http::Body;
use serde_json::{json, Value};
use uuid::Uuid;

pub async fn query(query: &str, token: &Option<String>) -> Result<Value, anyhow::Error> {
    let app = graph_api::App::new()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    let req_body = json!({
        "query": query,
    })
    .to_string();

    let mut request = lambda_http::Request::new(Body::from(req_body));

    *request.method_mut() = lambda_http::http::Method::POST;
    match token {
        Some(token) => {
            request.headers_mut().append(
                "Authorization",
                format!("Bearer {}", token).parse().unwrap(),
            );
        }
        None => {}
    }

    let res = app.respond(request).await.map_err(|e| anyhow::anyhow!(e))?;

    let body = res.body();

    Ok(serde_json::from_slice(body)?)
}

#[allow(dead_code)]
pub async fn login() -> Result<Option<String>, anyhow::Error> {
    login_specific("john@example.com").await
}

pub async fn login_specific(email: &str) -> Result<Option<String>, anyhow::Error> {
    let res = query(
        &format!(
            "mutation {{
                auth_token(
                    email: \"{}\",
                    password: \"password\"
                )
            }}",
            email
        ),
        &None,
    )
    .await?;

    if let Some(err) = &res["errors"][0].as_object() {
        panic!("{}", err["message"]);
    }

    Ok(res["data"]["auth_token"].as_str().map(|s| s.to_string()))
}

#[allow(dead_code)]
pub async fn create_user() -> Result<String, anyhow::Error> {
    let user_email = generate_random_email();
    let res = query(
        &format!(
            "mutation {{
                create_user(
                    email: \"{}\",
                    password: \"password\"
                    first_name: \"John\",
                    last_name: \"Doe\",
                    calling_code: \"AU\"
                    country_code: \"61\",
                    phone_number: \"000\",
                    referrer: null
                )
            }}",
            user_email
        ),
        &None,
    )
    .await?;
    assert_eq!(res["errors"], json!(null));

    Ok(user_email)
}

#[allow(dead_code)]
pub fn generate_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}

#[allow(dead_code)]
pub async fn create_user_with_admin_role() -> Result<String, anyhow::Error> {
    let user_email = create_user().await?;

    let res = query(
        "
        query {
            me {
                assign_role(role: \"admin\")
            }
        }
    ",
        &login_specific(&user_email).await?,
    )
    .await?;

    assert_eq!(res["errors"], json!(null));

    Ok(user_email)
}
