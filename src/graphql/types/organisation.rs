use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Organization {
    name: String,
}
