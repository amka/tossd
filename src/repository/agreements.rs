use log::debug;
use sea_orm::*;

use crate::agreements::Agreement;
use crate::models::agreements;

pub struct AgreementsRepository;

impl AgreementsRepository {
    pub async fn add(db: &DbConn, agreement: Agreement) -> Result<agreements::ActiveModel, DbErr> {
        debug!("AgreementsRepository::add <- {:?}", agreement);
        agreements::ActiveModel {
            internal_name: Set(agreement.internal_name),
            public_title: Set(agreement.public_title),
            public_text: Set(agreement.public_text),
            status: Set(agreement.status),
            author_id: Set(Option::from(agreement.author_id)),
            deleted: Set(agreement.deleted),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
            .save(db)
            .await
    }

    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Option<agreements::Model>, DbErr> {
        agreements::Entity::find_by_id(id).one(db).await
    }
}