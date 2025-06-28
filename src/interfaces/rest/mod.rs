use actix_web::web;
use user::user_controller;

pub mod identity;
pub mod user;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    user::user_controller::register_routes(cfg)
}
