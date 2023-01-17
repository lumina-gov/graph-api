use std::sync::Arc;

use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use juniper::FieldResult;

use crate::error::ErrorCode;
use crate::models::user::User;

pub struct GeneralContext {
    // Use your real database pool here.
    pub diesel_pool: Arc<Pool<AsyncPgConnection>>,
}


impl GeneralContext {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let postgrest_url: String = dotenv::var("DATABASE_URL")
            .expect("DATABASE_URL not set in .env");

        let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(postgrest_url);
        let pool = Pool::builder(config).build()?;

        Ok(Self {
            diesel_pool: Arc::new(pool),
        })
    }

    pub async fn new_unique_context(&self) -> UniqueContext {
        UniqueContext {
            diesel_pool: self.diesel_pool.clone(),
            user: None
        }
    }
}

pub struct UniqueContext {
    pub diesel_pool: Arc<Pool<AsyncPgConnection>>,
    pub user: Option<User>,
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for UniqueContext {}

impl UniqueContext {
    pub fn authenticated(&self) -> bool {
        self.user.is_some()
    }

    pub fn user(&self) -> FieldResult<User> {
        match &self.user {
            Some(user) => Ok(user.clone()),
            None => Err(ErrorCode::Unauthenticated.into()),
        }
    }

    pub fn has_role(&self, role: &str) -> bool {
        match &self.user {
            Some(user) => {
                match &user.role {
                    Some(user_role) => user_role == role,
                    None => false
                }
            },
            None => false
        }
    }
}