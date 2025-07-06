mod interfaces;
mod domain;
mod infrastructure;

use interfaces::rest;
use actix_cors::Cors;
use actix_web::{http, App, HttpServer};
use sea_orm::Database;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url).await.unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4200")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .app_data(actix_web::web::Data::new(db.clone())) // Share the db connection
            .wrap(cors)
            .configure(rest::register_route)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// ----------------------------------------------------------------------------------- //
// if you want migration, comment all above and uncomment all below. then "cargo run"! //
// ----------------------------------------------------------------------------------- //

// mod interfaces;
// mod domain;
// use migration::{Migrator, MigratorTrait};
// use sea_orm::Database;
// use std::env;

// #[tokio::main]
// async fn main() {
//     dotenvy::dotenv().ok();
//     let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let db = Database::connect(&db_url).await.unwrap();
//     Migrator::up(&db, None).await.unwrap();
// }

