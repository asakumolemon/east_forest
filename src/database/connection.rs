use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use crate::config::database_config::DatabaseConfig;


pub async fn create_pool(config: DatabaseConfig) -> Pool<Postgres> {
    let database_url = config.get_url();
    println!("Attempting to connect to database with URL: {}", database_url);
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .idle_timeout(config.idle_timeout)
        .max_lifetime(config.max_lifetime)
        .connect(&database_url)
        .await
        .map_err(|e| {
            eprintln!("Failed to create database pool: {}", e);
            e
        })
        .expect("Failed to create database pool.")
}

#[tokio::test]
async fn test_create_pool() {
    let config = DatabaseConfig::default();
    let pool = create_pool(config).await;
}