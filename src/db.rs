use std::env;

use sqlx::PgPool;

pub async fn establish_connection() -> Result<PgPool, Box<dyn std::error::Error>> {
    let db_url = env::var("DATABASE_URL").expect("Provide DATABASE_URL env var");
    Ok(PgPool::connect(db_url.as_str()).await?)
}