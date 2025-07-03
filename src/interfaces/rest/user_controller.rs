use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json;

#[get("/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"names":"asd"}))
}

#[post("/test")]
pub async fn test1() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"names":"test1"}))
}

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(test);
    cfg.service(test1);
}