use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::{json, Value};

struct AuthApp {
    name: String,
    description: String,
    created: DateTime<Utc>,
    redirect_hostnames: Vec<String>,
    scopes: Vec<String>,
    official: bool,
}

fn scopes() -> Value {
    json!({
        "profile": {
            "description": "View and edit your profile",
            "subscopes": {
                "read": {
                    "description": "View your profile",
                    "subscopes": {}
                },
                "edit": {
                    "description": "Edit your profile",
                    "subscopes": {}
                }
            }
        },
        "education": {
            "description": "View and edit your educational data",
            "subscopes": {
                "read": {
                    "description": "View your educational data",
                    "subscopes": {}
                },
                "edit": {
                    "description": "Edit your education data",
                    "subscopes": {}
                }
            }
        }
    })
}
