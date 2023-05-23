use crate::schema::unit_progress::{ActiveModel, Column, Entity, Model};

pub type UnitProgress = Model;
pub type UnitProgressEntity = Entity;
pub type UnitProgressActiveModel = ActiveModel;
pub type UnitProgressColumn = Column;

pub type UnitStatus = crate::schema::sea_orm_active_enums::UnitStatus;
