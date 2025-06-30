use sea_orm::Database;
use migration::{Migrator, MigratorTrait};


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Database::connect(&db_url).await.unwrap();

    Migrator::up(&db, None).await.unwrap();

    println!("Migration ran successfully!");
}