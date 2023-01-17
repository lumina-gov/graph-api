use juniper::GraphQLObject;
use serde::{Serialize, Deserialize};


#[derive(GraphQLObject, Clone, Debug, Serialize, Deserialize)]
pub struct CrackSeconds {
    pub guesses: i32,
    pub seconds: f64,
    pub string: String,
}
