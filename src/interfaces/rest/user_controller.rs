use actix_web::{get, post, put, delete, web, HttpRequest, Responder, HttpResponse};
use serde_json::json;
use crate::domain::user::user_service::UserService;
use crate::interfaces::dtos::user_dto::{UserDto, CreateUserDto};
use sea_orm::DatabaseConnection;


#[get("/users")]
pub async fn get_all_users(data: web::Data<DatabaseConnection>, _req: HttpRequest) -> impl Responder {
    match UserService::get_all_users(&data).await {
        Ok(users) => {
            let dtos: Vec<UserDto> = users.into_iter().map(|user| user.into()).collect();
            web::Json(json!({ "status": 200, "data": dtos, "errorMessage": "" }))
        },
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() }))
    }
}

#[get("/users/{id}")]
pub async fn get_user_by_id(data: web::Data<DatabaseConnection>, _req: HttpRequest, id: web::Path<i32>) -> impl Responder {
    match UserService::get_user_by_id(&data, id.into_inner()).await {
        Ok(Some(user)) => {
            let dto: UserDto = user.into();
            web::Json(json!({ "status": 200, "data": [dto], "errorMessage": "" }))
        },
        Ok(None) => web::Json(json!({ "status": 200, "data": [], "errorMessage": "Not found" })),
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() }))
    }
}

#[get("/users/email/{email}")]
pub async fn get_user_by_email(data: web::Data<DatabaseConnection>, _req: HttpRequest, email: web::Path<String>) -> impl Responder {
    match UserService::get_user_by_email(&data, &email.into_inner()).await {
        Ok(Some(user)) => {
            let dto: UserDto = user.into();
            web::Json(json!({ "status": 200, "data": [dto], "errorMessage": "" }))
        },
        Ok(None) => web::Json(json!({ "status": 200, "data": [], "errorMessage": "Not found" })),
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() }))
    }
}

#[get("/users/display/{display_id}")]
pub async fn get_user_by_display_id(data: web::Data<DatabaseConnection>, _req: HttpRequest, display_id: web::Path<String>) -> impl Responder {
    match UserService::get_user_by_display_id(&data, &display_id.into_inner()).await {
        Ok(Some(user)) => {
            let dto: UserDto = user.into();
            web::Json(json!({ "status": 200, "data": [dto], "errorMessage": "" }))
        },
        Ok(None) => web::Json(json!({ "status": 200, "data": [], "errorMessage": "Not found" })),
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() }))
    }
}

#[get("/users/search/{name}")]
pub async fn get_users_by_name(data: web::Data<DatabaseConnection>, _req: HttpRequest, name: web::Path<String>) -> impl Responder {
    match UserService::get_user_by_name(&data, &name.into_inner()).await {
        Ok(users) => {
            let dtos: Vec<UserDto> = users.into_iter().map(|user| user.into()).collect();
            web::Json(json!({ "status": 200, "data": dtos, "errorMessage": "" }))
        },
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() }))
    }
}

#[post("/users")]
pub async fn create_user(data: web::Data<DatabaseConnection>, _req: HttpRequest, payload: web::Json<CreateUserDto>) -> impl Responder {
    match UserService::create_user(
        &data,
        payload.name.clone(),
        payload.password.clone(),
        payload.email.clone(),
        payload.email_confirmed,
        payload.user_display_id.clone(),
        payload.balance,
        payload.is_active,
        payload.role_id,
        payload.group_id,
        payload.created_date,
    ).await {
        Ok(user) => {
            let dto: UserDto = user.into();
            web::Json(json!({ "status": 200, "data": [dto], "errorMessage": "" }))
        },
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() }))
    }
}

#[put("/users/{id}")]
pub async fn update_user(data: web::Data<DatabaseConnection>, _req: HttpRequest, id: web::Path<i32>, payload: web::Json<CreateUserDto>) -> impl Responder {
    match UserService::update_user(
        &data,
        id.into_inner(),
        Some(payload.name.clone()),
        Some(payload.password.clone()),
        Some(payload.email.clone()),
        Some(payload.email_confirmed),
        Some(payload.user_display_id.clone()),
        Some(payload.balance),
        Some(payload.is_active),
        Some(payload.role_id),
        Some(payload.group_id),
    ).await {
        Ok(Some(user)) => {
            let dto: UserDto = user.into();
            web::Json(json!({ "status": 200, "data": [dto], "errorMessage": "" }))
        },
        Ok(None) => web::Json(json!({ "status": 200, "data": [], "errorMessage": "Not found" })),
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() }))
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(data: web::Data<DatabaseConnection>, _req: HttpRequest, id: web::Path<i32>) -> impl Responder {
    match UserService::delete_user(&data, id.into_inner()).await {
        Ok(true) => web::Json(json!({ "status": 200, "data": [], "errorMessage": "" })),
        Ok(false) => web::Json(json!({ "status": 200, "data": [], "errorMessage": "Not found" })),
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() }))
    }
}

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
    cfg.service(get_user_by_id);
    cfg.service(get_user_by_email);
    cfg.service(get_user_by_display_id);
    cfg.service(get_users_by_name);
    cfg.service(create_user);
    cfg.service(update_user);
    cfg.service(delete_user);
}