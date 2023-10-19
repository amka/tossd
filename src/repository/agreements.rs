use log::debug;
use sea_orm::*;

use crate::agreements::Agreement;
use crate::models::{agreement, agreement_versions};

pub struct AgreementsRepository;

impl AgreementsRepository {
    pub async fn add(db: &DbConn, agreement: Agreement) -> Result<agreement::ActiveModel, DbErr> {
        debug!("AgreementsRepository::add <- {:?}", agreement);
        agreement::ActiveModel {
            id: Default::default(),
            inner_title: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
            author_id: Default::default(),
            ..Default::default()
        }
            .save(db)
            .await
    }

    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Option<agreement::Model>, DbErr> {
        agreement::Entity::find_by_id(id).one(db).await
    }
}