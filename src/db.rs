use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use dotenvy::dotenv;

pub async fn init_db() -> PgPool {
    dotenv().ok(); 

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create PostgreSQL connection pool")
}