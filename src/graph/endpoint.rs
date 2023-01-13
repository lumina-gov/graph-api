use juniper::Variables;
use serde::{Deserialize, Deserializer, Serialize};

use super::{context::create_context, resolvers::create_schema};

#[derive(Serialize, Deserialize)]
struct GraphQLRequest {
    #[serde(deserialize_with = "parse_variables")]
    variables: Variables, // or Variables::new()
    query: String,
    #[allow(non_snake_case)]
    operationName: Option<String>, // or None
}

fn run_query(query_string: &str, variables: &Variables, operation_name: Option<&str>) -> String {
    let context = create_context();
    let schema = create_schema();

    let (res, _errors) =
        juniper::execute_sync(query_string, operation_name, &schema, &variables, &context).unwrap();

    res.to_string()
}

pub fn run_request(request_string: &str) -> String {
    let request: GraphQLRequest = serde_json::from_str(&request_string).unwrap();
    run_query(
        &request.query,
        &request.variables,
        request.operationName.as_deref(),
    )
}

fn parse_variables<'de, D>(d: D) -> Result<Variables, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or(Variables::new()))
}
