use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(GraphQLObject, Clone, Debug, Serialize, Deserialize)]
pub struct CrackSeconds {
    pub guesses: f64,
    pub seconds: f64,
    pub string: String,
}
