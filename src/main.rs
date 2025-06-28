mod application;
mod domain;
mod infrastructure;
mod interfaces;

use actix_web::{App, HttpServer};
// use domain::identity::services::{user_service::UserService, role_service::RoleService};
use interfaces::rest::register_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let db = database::init_db().await;
    
    HttpServer::new(move || {
        App::new()
        .configure(register_routes)
            // .app_data(web::Data::new(UserService::new(&db)))
            // .app_data(web::Data::new(RoleService::new(&db)))
            // .configure(crate::interfaces::rest::register_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}