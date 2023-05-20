//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use async_graphql::SimpleObject;
use chrono::serde::ts_milliseconds;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
#[graphql(complex)]
#[graphql(rename_fields = "snake_case", rename_args = "snake_case")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub email: String,
    #[serde(with = "ts_milliseconds")]
    pub joined: DateTimeWithTimeZone,
    #[graphql(skip)]
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub calling_code: String,
    pub country_code: String,
    pub phone_number: String,
    pub role: Option<String>,
    #[graphql(skip)]
    pub referrer: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::question_assessments::Entity")]
    QuestionAssessments,
    #[sea_orm(has_many = "super::unit_progress::Entity")]
    UnitProgress,
}

impl Related<super::question_assessments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QuestionAssessments.def()
    }
}

impl Related<super::unit_progress::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UnitProgress.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub type User = Model;
pub type UserEntity = Entity;
pub type UserActiveModel = ActiveModel;
