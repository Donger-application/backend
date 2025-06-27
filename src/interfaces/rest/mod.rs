pub mod identity;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    identity::user_controller::register_routes(cfg);
    identity::role_controller::register_routes(cfg);
}