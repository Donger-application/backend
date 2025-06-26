use actix_web::{App, HttpServer, web};
use crate::domain::identity::services::{user_service::UserService, role_service::RoleService};
use crate::infrastructure::database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = database::init_db().await;
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(UserService::new(&db)))
            .app_data(web::Data::new(RoleService::new(&db)))
            .configure(crate::interfaces::rest::register_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}