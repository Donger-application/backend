use actix_web::{get, post, web, HttpResponse, Responder};
use validator::Validate;
use crate::domain::identity::services::user_service::UserService;
use crate::interfaces::dtos::identity::user::{UserDto, CreateUserDto};

#[get("/users/{id}")]
pub async fn get_user_data(
    id: web::Path<i32>,
    user_service: web::Data<UserService>,
) -> impl Responder {
    let user_id = id.into_inner();
    match user_service.get_user_data(user_id).await {
        Ok(user) => HttpResponse::Ok().json(UserDto {
            id: user.id,
            name: user.name,
            role_id: user.role_id,
            is_active: user.is_active,
            register_date: user.register_date,
        }),
        Err(_) => HttpResponse::NotFound().body("User not found"),
    }
}

#[post("/users")]
pub async fn create_user(
    user_dto: web::Json<CreateUserDto>,
    user_service: web::Data<UserService>,
) -> impl Responder {
    if let Err(e) = user_dto.validate() {
        return HttpResponse::BadRequest().body(format!("Validation error: {}", e));
    }
    match user_service
        .create_user(user_dto.name.clone(), user_dto.role_id)
        .await
    {
        Ok(user) => HttpResponse::Created().json(UserDto {
            id: user.id,
            name: user.name,
            role_id: user.role_id,
            is_active: user.is_active,
            register_date: user.register_date,
        }),
        Err(e) => HttpResponse::BadRequest().body(format!("Error creating user: {}", e)),
    }
}

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_data);
    cfg.service(create_user);
}