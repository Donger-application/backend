mod domain;
mod infrastructure;
mod interfaces;

use actix_cors::Cors;
use actix_web::{http, App, HttpServer};
use interfaces::rest;
use sqlx::PgPool;
use sqlx_clean_querybuilder::query_builder::PostgreSqlQueryBuilder;
// use sea_orm::Database;
use std::env;

use crate::interfaces::dtos::user_dto::UserDto;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPool::connect(&db_url).await.unwrap();

    let qq = PostgreSqlQueryBuilder::select().columns(&[]).table("user", None).build();

    let users: Vec<UserDto> = sqlx::query_as::<_, UserDto>(&qq)
    .fetch_all(&db)
    .await
    .unwrap();

    println!("All users: {:#?}", users);

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

// use sqlx::{postgres::PgPoolOptions};
// use std::env;

// #[tokio::main]
// async fn main() -> Result<(), sqlx::Error> {
//     dotenvy::dotenv().ok();
//     let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

//     // Create SQLx connection pool
//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect(&db_url)
//         .await?;

//     // Run migrations (from ./migrations)
//     sqlx::migrate!("./migrations").run(&pool).await?;

//     println!("Database migrated successfully!");
//     Ok(())
// }