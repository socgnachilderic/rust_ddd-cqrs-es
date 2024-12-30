use std::sync::Arc;

use dotenvy::var;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

mod models;
mod repositories;

pub use repositories::*;

pub type Pool = PgPool;

pub async fn establish_connection() -> sqlx::Result<Arc<PgPool>> {
    let database_url = var("DATABASE_URL")
        .map_err(|e| format!("Failed to get DATABASE_URL: {}", e))
        .unwrap();

    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .map(Arc::new)
}
