use std::collections::HashMap;

use crate::graphql::types::auth_apps::AuthApp;
use async_graphql::Object;
use chrono::TimeZone;
use lazy_static::lazy_static;

lazy_static! {
    static ref APPS: HashMap<&'static str, AuthApp> = {
        let mut apps = std::collections::HashMap::new();

        apps.insert(
            "lumina-university",
            AuthApp {
                name: "Lumina University".to_string(),
                scopes: vec!["profile:read".into(), "billing".into(), "education".into()],
                redirect_hostnames: vec!["luminauniversity.earth".into(), "localhost".into()],
                created: chrono::Utc
                    .with_ymd_and_hms(2023, 5, 25, 0, 0, 0)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                description: "The next generation of education".to_string(),
                official: true,
            },
        );

        apps
    };
}

#[derive(Default)]
pub struct AuthAppsQuery;

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl AuthAppsQuery {
    async fn auth_app(&self, slug: String) -> async_graphql::Result<Option<&'static AuthApp>> {
        return Ok(APPS.get(slug.as_str()));
    }
}
