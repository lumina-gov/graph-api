use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject, GraphQLUnion};
use uuid::Uuid;

#[derive(GraphQLEnum)]
pub enum ApplicationType {
    Citizenship,
    Pioneer,
    Organization,
}

#[derive(GraphQLEnum)]
pub enum ApplicationStatus {
    Received,
    Deliberation,
    Accepted,
    Rejected,
}

#[derive(GraphQLObject)]
pub struct Application {
    id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    application_type: ApplicationType,
    bson: String,
    status: ApplicationStatus,
}
