//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "agreement_acceptance_status")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub user_id: i64,
    pub provider_id: i32,
    pub agreement_id: i32,
    pub version: i32,
    pub accepted: bool,
    pub accepted_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::agreement::Entity",
        from = "Column::AgreementId",
        to = "super::agreement::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Agreement,
}

impl Related<super::agreement::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Agreement.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
