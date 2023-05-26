use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};

pub mod application;
pub mod course;
pub mod organisation;
pub mod question_assessment;
pub mod unit_progress;
pub mod user;

pub struct Void;

#[Scalar]
impl ScalarType for Void {
    fn parse(value: Value) -> InputValueResult<Self> {
        Err(InputValueError::custom(
            "you cannot just input void into the api",
        ))
    }

    fn to_value(&self) -> Value {
        Value::Null
    }
}
