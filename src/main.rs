// src/main.rs

use actix_web::{get, App, HttpServer, Responder};
use dotenvy::dotenv;

// Declare our top-level modules
mod application;
mod domain;
mod infrastructure;
mod interfaces;

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, world from my_backend_app!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // You would typically initialize your database connection pool here.
    // For now, we'll just demonstrate the web server setup.
    // let database_url = std::env::var("DATABASE_URL")
    //     .expect("DATABASE_URL must be set in .env file");
    // let pool = infrastructure::database::establish_connection(&database_url)
    //     .expect("Failed to create connection pool");

    println!("Starting server on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            // Here you would pass application state, like the database pool
            // .app_data(web::Data::new(pool.clone()))
            .service(hello)
            // Register your REST controllers here
            // .service(interfaces::rest::auth_controller::register)
            // .service(interfaces::rest::user_controller::get_user)
            // Register your WebSocket route here
            // .service(web::resource("/ws/").to(interfaces::websocket::ws_session::ws_index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

