use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use crate::config::database_config::DatabaseConfig;


pub async fn create_pool(config: DatabaseConfig) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .idle_timeout(config.idle_timeout)
        .max_lifetime(config.max_lifetime)
        .connect(&config.get_url())
        .await
        .expect("Failed to create database pool.")
}