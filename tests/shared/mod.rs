mod custom_postgres;
use std::{fs::read_to_string, sync::Arc};

use crate::shared::custom_postgres::Postgres;
use graph_api::{App, SECRET_VARIABLES};
use lambda_http::Body;
use lazy_static::lazy_static;
use sea_orm::{ConnectionTrait, Database};
use serde_json::{json, Value};
use testcontainers::clients::Cli;
use testcontainers::Container;

lazy_static! {
    static ref DOCKER_CLIENT: Cli = Cli::docker();
}

#[allow(dead_code)]
pub struct SharedApp {
    app: App,
    postgres_container: Container<'static, Postgres>,
}

impl SharedApp {
    pub async fn init() -> SharedApp {
        let postgres = Postgres::default();
        let postgres_container: Container<'static, Postgres> = DOCKER_CLIENT.run(postgres);
        let postgres_url = format!(
            "postgresql://test:test@localhost:{}/postgres",
            postgres_container.get_host_port_ipv4(5432)
        );

        std::env::set_var("TEST_POSTGRES_URL", &postgres_url);

        {
            let db = Database::connect(&postgres_url).await.unwrap();
            let schema = read_to_string("schema.sql").unwrap();
            db.execute_unprepared(&schema).await.unwrap();
        }

        let app = graph_api::App::new(Some(postgres_url))
            .await
            .map_err(|e| anyhow::anyhow!(e))
            .unwrap();

        SharedApp {
            app,
            postgres_container,
        }
    }

    pub fn get_db_url(&self) -> String {
        format!(
            "postgresql://test:test@localhost:{}/postgres",
            self.postgres_container.get_host_port_ipv4(5432)
        )
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
        self.login_specific_with_scopes(email, vec!["*"]).await
    }

    pub async fn login_specific_with_scopes(
        &self,
        email: &str,
        scopes: Vec<&str>,
    ) -> Result<Option<String>, anyhow::Error> {
        let res = self
            .query(
                &format!(
                    "mutation {{
                auth_token(
                    email: \"{}\",
                    password: \"password\",
                    scopes: [{}]
                    app_secret: \"{}\"
                )
            }}",
                    email,
                    scopes
                        .iter()
                        .map(|s| format!("\"{}\"", s))
                        .collect::<Vec<String>>()
                        .join(","),
                    &SECRET_VARIABLES.app_secret
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
        let user_email: String = "gov@lumina.earth".to_owned();
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
