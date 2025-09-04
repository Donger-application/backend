use actix_web::{web};

// #[get("/role")]
// pub async fn get_all_roles(
//     data: web::Data<DatabaseConnection>,
//     _req: HttpRequest,
// ) -> impl Responder {
//     match RoleService::get_all_roles(&data).await {
//         Ok(roles) => {
//             let dtos: Vec<RoleDto> = roles.into_iter().map(|role| role.into()).collect();
//             web::Json(ApiResponse::new(200, dtos, ""))
//         }
//         Err(e) => web::Json(ApiResponse::new(500, Vec::<RoleDto>::new(), e.to_string())),
//     }
// }

// #[get("/role/{id}")]
// pub async fn get_role_by_id(
//     data: web::Data<DatabaseConnection>,
//     _req: HttpRequest,
//     id: web::Path<i32>,
// ) -> impl Responder {
//     match RoleService::get_role_by_id(&data, id.into_inner()).await {
//         Ok(Some(role)) => {
//             let dto: RoleDto = role.into();
//             web::Json(ApiResponse::new(200, vec![dto], ""))
//         }
//         Ok(None) => web::Json(ApiResponse::new(404, Vec::<RoleDto>::new(), "Not found")),
//         Err(e) => web::Json(ApiResponse::new(500, Vec::<RoleDto>::new(), e.to_string())),
//     }
// }

// #[get("/role/search/{name}")]
// pub async fn get_role_by_name(
//     data: web::Data<DatabaseConnection>,
//     _req: HttpRequest,
//     name: web::Path<String>,
// ) -> impl Responder {
//     match RoleService::get_role_by_name(&data, &name.into_inner()).await {
//         Ok(roles) => {
//             let dtos: Vec<RoleDto> = roles.into_iter().map(|role| role.into()).collect();
//             web::Json(ApiResponse::new(200, dtos, ""))
//         }
//         Err(e) => web::Json(ApiResponse::new(500, Vec::<RoleDto>::new(), e.to_string())),
//     }
// }

// #[post("/role")]
// pub async fn create_role(
//     data: web::Data<DatabaseConnection>,
//     _req: HttpRequest,
//     payload: web::Json<CreateRoleDto>,
// ) -> impl Responder {
//     match RoleService::create_role(&data, payload.name.clone()).await {
//         Ok(role) => {
//             let dto: RoleDto = role.into();
//             web::Json(ApiResponse::new(200, vec![dto], ""))
//         }
//         Err(e) => web::Json(ApiResponse::new(500, Vec::<RoleDto>::new(), e.to_string())),
//     }
// }

// #[put("/role/{id}")]
// pub async fn update_role(
//     data: web::Data<DatabaseConnection>,
//     _req: HttpRequest,
//     id: web::Path<i32>,
//     payload: web::Json<CreateRoleDto>,
// ) -> impl Responder {
//     match RoleService::update_role(&data, id.into_inner(), payload.name.clone()).await {
//         Ok(Some(role)) => {
//             let dto: RoleDto = role.into();
//             web::Json(ApiResponse::new(200, vec![dto], ""))
//         }
//         Ok(None) => web::Json(ApiResponse::new(404, Vec::<RoleDto>::new(), "Not found")),
//         Err(e) => web::Json(ApiResponse::new(500, Vec::<RoleDto>::new(), e.to_string())),
//     }
// }

// #[delete("/role/{id}")]
// pub async fn delete_role(
//     data: web::Data<DatabaseConnection>,
//     _req: HttpRequest,
//     id: web::Path<i32>,
// ) -> impl Responder {
//     match RoleService::delete_role(&data, id.into_inner()).await {
//         Ok(true) => web::Json(ApiResponse::new(200, Vec::<RoleDto>::new(), "")),
//         Ok(false) => web::Json(ApiResponse::new(404, Vec::<RoleDto>::new(), "Not found")),
//         Err(e) => web::Json(ApiResponse::new(500, Vec::<RoleDto>::new(), e.to_string())),
//     }
// }

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    // cfg.service(get_all_roles);
    // cfg.service(get_role_by_id);
    // cfg.service(get_role_by_name);
    // cfg.service(create_role);
    // cfg.service(update_role);
    // cfg.service(delete_role);
}
