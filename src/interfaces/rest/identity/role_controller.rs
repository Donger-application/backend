use actix_web::{get, post, web, HttpResponse, Responder};
use validator::Validate;
use crate::domain::identity::services::role_service::RoleService;
use crate::interfaces::dtos::identity::role::{RoleDto, CreateRoleDto};

#[get("/roles/{id}")]
pub async fn get_role_data(
    id: web::Path<i32>,
    role_service: web::Data<RoleService>,
) -> impl Responder {
    let role_id = id.into_inner();
    match role_service.get_role_data(role_id).await {
        Ok(role) => HttpResponse::Ok().json(RoleDto {
            id: role.id,
            name: role.name,
        }),
        Err(_) => HttpResponse::NotFound().body("Role not found"),
    }
}

#[post("/roles")]
pub async fn create_role(
    role_dto: web::Json<CreateRoleDto>,
    role_service: web::Data<RoleService>,
) -> impl Responder {
    if let Err(e) = role_dto.validate() {
        return HttpResponse::BadRequest().body(format!("Validation error: {}", e));
    }
    match role_service.create_role(role_dto.name.clone()).await {
        Ok(role) => HttpResponse::Created().json(RoleDto {
            id: role.id,
            name: role.name,
        }),
        Err(e) => HttpResponse::BadRequest().body(format!("Error creating role: {}", e)),
    }
}

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_role_data);
    cfg.service(create_role);
}