//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use async_graphql::{InputObject, SimpleObject};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject, InputObject)]
#[sea_orm(table_name = "oauth_apps")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub client_id: String,
    pub redirect_uris: String,
    pub app_name: String,
    pub client_secret: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}