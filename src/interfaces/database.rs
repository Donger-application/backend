use sea_orm::{Database, DatabaseConnection};
use dotenvy::dotenv;
use std::env;

pub async fn init_db() -> DatabaseConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}