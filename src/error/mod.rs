use async_graphql::ErrorExtensions;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct APIError {
    code: String,
    message: String,
}

impl Error for APIError {}
impl Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl APIError {
    pub fn new(code: &str, message: &str) -> Self {
        tracing::error!("APIError::{}: {}", code, message);
        let err = Self {
            code: code.to_string(),
            message: message.to_string(),
        };

        err.extend_with(|_, e| e.set("code", err.code.clone()));

        err
    }

    pub fn new_with_detail(code: &str, message: &str, detail: &str) -> Self {
        tracing::error!("APIError::{}: {}\n{}", code, message, detail);
        let err = Self {
            code: code.to_string(),
            message: message.to_string(),
        };

        err.extend_with(|_, e| e.set("code", err.code.clone()));

        err
    }
}
