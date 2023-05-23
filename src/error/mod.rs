use async_graphql::ErrorExtensions;

pub fn new_err(code: &str, message: &str) -> async_graphql::Error {
    tracing::error!("APIError::{}: {}", code, message);

    async_graphql::Error::new(message).extend_with(|_, e| e.set("code", code))
}

pub fn new_err_with_detail(code: &str, message: &str, detail: &str) -> async_graphql::Error {
    tracing::error!("APIError::{}: {}\n{}", code, message, detail);

    async_graphql::Error::new(message)
        .extend_with(|_, e| e.set("detail", detail))
        .extend_with(|_, e| e.set("code", code))
}
