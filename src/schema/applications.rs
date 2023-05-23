//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use async_graphql::{InputObject, SimpleObject};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject, InputObject)]
#[sea_orm(table_name = "applications")]
#[graphql]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(column_type = "JsonBinary")]
    pub application: Json,
    pub application_type: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}