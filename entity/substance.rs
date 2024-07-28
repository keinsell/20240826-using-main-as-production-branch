//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.7

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "substance")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ingestion::Entity")]
    Ingestion,
}

impl Related<super::ingestion::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ingestion.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
