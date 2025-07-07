use crate::domain::role::role_service::RoleService;
use crate::interfaces::dtos::role_dto::{CreateRoleDto, RoleDto};
use actix_web::{get, post, put, web, HttpRequest, Responder};
use sea_orm::DatabaseConnection;
use serde_json::json;


#[get("/role")]
pub async fn get_all_roles(
    data: web::Data<DatabaseConnection>,
    _req: HttpRequest,
) -> impl Responder {
    match RoleService::get_all_roles(&data).await {
        Ok(role) => {
            let dtos: Vec<RoleDto> = role
                .into_iter()
                .map(|r| RoleDto {
                    id: r.id,
                    name: r.name,
                })
                .collect();
            web::Json(json!({ "status": 200, "data": dtos, "errorMessage": "" }))
        }
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() })),
    }
}

#[get("/role/{id}")]
pub async fn get_role_by_id(
    data: web::Data<DatabaseConnection>,
    _req: HttpRequest,
    id: web::Path<i32>,
) -> impl Responder {
    match RoleService::get_role_by_id(&data, id.into_inner()).await {
        Ok(Some(r)) => web::Json(
            json!({ "status": 200, "data": [RoleDto { id: r.id, name: r.name }], "errorMessage": "" }),
        ),
        Ok(None) => web::Json(json!({ "status": 200, "data": [], "errorMessage": "Not found" })),
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() })),
    }
}

#[get("/role/by-name/{name}")]
pub async fn get_role_by_name(
    data: web::Data<DatabaseConnection>,
    _req: HttpRequest,
    name: web::Path<String>,
) -> impl Responder {
    match RoleService::get_role_by_name(&data, &name.into_inner()).await {
        Ok(role) => {
            let dtos: Vec<RoleDto> = role
                .into_iter()
                .map(|r| RoleDto {
                    id: r.id,
                    name: r.name,
                })
                .collect();
            web::Json(json!({ "status": 200, "data": dtos, "errorMessage": "" }))
        }
        // Ok(None) => web::Json(json!({ "status": 200, "data": [], "errorMessage": "Not found" })),
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() })),
    }
}

#[utoipa::path(
    post,
    path = "/role",
    request_body(content = CreateRoleDto, content_type = "application/json"),
    responses(
        (status = 200, description = "Role created successfully", body = RoleDto),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[post("/role")]
pub async fn create_role(
    data: web::Data<DatabaseConnection>,
    _req: HttpRequest,
    payload: web::Json<CreateRoleDto>,
) -> impl Responder {
    match RoleService::create_role(&data, payload.name.clone()).await {
        Ok(r) => web::Json(
            json!({ "status": 200, "data": [RoleDto { id: r.id, name: r.name }], "errorMessage": "" }),
        ),
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() })),
    }
}

#[put("/role/{id}")]
pub async fn update_role(
    data: web::Data<DatabaseConnection>,
    _req: HttpRequest,
    id: web::Path<i32>,
    payload: web::Json<CreateRoleDto>,
) -> impl Responder {
    match RoleService::update_role(&data, id.into_inner(), payload.name.clone()).await {
        Ok(Some(r)) => web::Json(
            json!({ "status": 200, "data": [RoleDto { id: r.id, name: r.name }], "errorMessage": "" }),
        ),
        Ok(None) => web::Json(json!({ "status": 200, "data": [], "errorMessage": "Not found" })),
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() })),
    }
}

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_roles);
    cfg.service(get_role_by_id);
    cfg.service(get_role_by_name);
    cfg.service(create_role);
    cfg.service(update_role);
}