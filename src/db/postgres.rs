use sqlx::pool::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::Postgres;

use crate::config::AppConfig;

pub async fn get_connection(config: &AppConfig) -> Pool<Postgres> {
      PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database")
}