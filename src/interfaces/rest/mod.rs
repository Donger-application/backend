use actix_web::web;
pub mod role_controller;
pub mod user_controller;

pub fn register_route(cfg: &mut web::ServiceConfig) {
    role_controller::register_routes(cfg);
    user_controller::register_routes(cfg);
    // Add other controllers here
}
