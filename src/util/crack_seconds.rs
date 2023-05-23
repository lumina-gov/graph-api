use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
#[graphql(rename_fields = "snake_case", rename_args = "snake_case")]
pub struct CrackSeconds {
    pub guesses: f64,
    pub seconds: f64,
    pub string: String,
}
