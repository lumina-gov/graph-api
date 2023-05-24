mod custom_postgres;
use std::{fs::read_to_string, sync::Arc};

use crate::shared::custom_postgres::Postgres;
use graph_api::App;
use lambda_http::Body;
use sea_orm::{ConnectionTrait, Database};
use serde_json::{json, Value};
use testcontainers::Container;
use uuid::Uuid;
#[allow(dead_code)]
pub struct SharedApp<'a> {
    app: App,
    postgres_container: Container<'a, Postgres>,
    postgres_url: String,
}

impl<'a> SharedApp<'a> {
    pub async fn init(docker_client: &'a testcontainers::clients::Cli) -> SharedApp<'a> {
        let arc_docker_client = Arc::new(docker_client);
        let postgres = Postgres::default();
        let postgres_container: Container<'a, Postgres> = arc_docker_client.run(postgres);
        let postgres_url = format!(
            "postgresql://test:test@localhost:{}/postgres",
            postgres_container.get_host_port_ipv4(5432)
        );
        {
            let db = Database::connect(&postgres_url).await.unwrap();
            let schema = read_to_string("schema.sql").unwrap();
            db.execute_unprepared(&schema).await.unwrap();
        }
        let app = graph_api::App::new(&postgres_url, &dotenv::var("OPENAI_KEY").unwrap(), "secret")
            .await
            .map_err(|e| anyhow::anyhow!(e))
            .unwrap();

        SharedApp {
            app,
            postgres_container,
            postgres_url,
        }
    }
    pub async fn query(&self, query: &str, token: &Option<String>) -> Result<Value, anyhow::Error> {
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

        let res = self
            .app
            .respond(request)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        let body = res.body();

        Ok(serde_json::from_slice(body)?)
    }

    #[allow(dead_code)]
    pub async fn login(&self) -> Result<Option<String>, anyhow::Error> {
        self.login_specific("john@example.com").await
    }

    pub async fn login_specific(&self, email: &str) -> Result<Option<String>, anyhow::Error> {
        let res = self
            .query(
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
    pub async fn create_user(&self) -> Result<String, anyhow::Error> {
        let user_email = "cat@example.com".to_owned();
        let res = self
            .query(
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
    pub async fn create_user_with_admin_role(&self) -> Result<String, anyhow::Error> {
        let user_email = self.create_user().await?;

        let res = self
            .query(
                "
        query {
            me {
                assign_role(role: \"admin\")
            }
        }
    ",
                &self.login_specific(&user_email).await?,
            )
            .await?;

        assert_eq!(res["errors"], json!(null));

        Ok(user_email.to_string())
    }
}
