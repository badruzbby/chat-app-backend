use sqlx::{
    postgres::PgPoolOptions,
    postgres::PgPool,
};
use anyhow::Result;

use crate::config::get_database_url;

pub async fn create_db_pool() -> Result<PgPool> {
    let database_url = get_database_url();
    
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;
    
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    
    Ok(pool)
}

#[allow(dead_code)]
async fn create_pg_pool(url: &str) -> Result<PgPool> {
    Ok(PgPoolOptions::new().connect(url).await?)
}

#[allow(dead_code)]
async fn create_sqlite_pool(url: &str) -> Result<sqlx::SqlitePool> {
    Ok(sqlx::sqlite::SqlitePoolOptions::new().connect(url).await?)
} 