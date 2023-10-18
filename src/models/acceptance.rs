use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct AgreementAcceptance {
    pub id: i32,
    pub agreement_id: String,
    pub user_id: String,
    pub accepted: bool,
    pub accepted_at: chrono::DateTime<chrono::Utc>,
}
