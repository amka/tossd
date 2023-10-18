use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Agreement {
    pub id: i32,
    pub internal_name: String,
    pub public_title: String,
    pub public_text: String,
    pub status: String,
    pub published_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub author_id: String,
    pub deleted: bool,
}
