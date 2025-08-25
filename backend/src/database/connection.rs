use sqlx::{Pool, Postgres, PgPool};
use std::env;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://username:password@localhost:5432/xforcesolutions".to_string());
    
    PgPool::connect(&database_url).await
}