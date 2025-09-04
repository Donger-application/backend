use actix_web::web;
pub mod role_controller;
pub mod user_controller;
pub mod product_controller;
pub mod meal_controller;
pub mod invoice_controller;
pub mod invoice_details_controller;

pub fn register_route(cfg: &mut web::ServiceConfig) {
    role_controller::register_routes(cfg);
    user_controller::register_routes(cfg);
    product_controller::register_routes(cfg);
    meal_controller::register_routes(cfg);
    invoice_controller::register_routes(cfg);
    invoice_details_controller::register_routes(cfg);
    // Add other controllers here
}
