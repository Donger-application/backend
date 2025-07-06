use actix_web::{get, post, put, web, HttpRequest, Responder, HttpResponse};
use serde_json::json;
use crate::domain::role::role_service::RoleService;
use crate::interfaces::dtos::role_dto::{RoleDto, CreateRoleDto};
use sea_orm::DatabaseConnection;

/// REST API endpoints for role management.
/// 
/// This module provides HTTP endpoints for performing CRUD operations on roles.
/// All endpoints return JSON responses with a consistent format:
/// ```json
/// {
///   "status": 200,
///   "data": [...],
///   "errorMessage": ""
/// }
/// ```
/// 
/// # Endpoints
/// 
/// * `GET /role` - Get all roles
/// * `GET /role/{id}` - Get role by ID
/// * `GET /role/by-name/{name}` - Search roles by name
/// * `POST /role` - Create a new role
/// * `PUT /role/{id}` - Update an existing role

/// Retrieves all roles from the database.
/// 
/// This endpoint fetches all role records and returns them as a JSON array.
/// 
/// # Request
/// 
/// `GET /role`
/// 
/// # Response
/// 
/// ## Success (200)
/// ```json
/// {
///   "status": 200,
///   "data": [
///     {
///       "id": 1,
///       "name": "admin"
///     },
///     {
///       "id": 2,
///       "name": "user"
///     }
///   ],
///   "errorMessage": ""
/// }
/// ```
/// 
/// ## Error (500)
/// ```json
/// {
///   "status": 500,
///   "data": [],
///   "errorMessage": "Database error message"
/// }
/// ```
#[get("/role")]
pub async fn get_all_roles(data: web::Data<DatabaseConnection>, _req: HttpRequest) -> impl Responder {
    match RoleService::get_all_roles(&data).await {
        Ok(role) => {
            let dtos: Vec<RoleDto> = role.into_iter().map(|r| RoleDto { id: r.id, name: r.name }).collect();
            web::Json(json!({ "status": 200, "data": dtos, "errorMessage": "" }))
        },
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() }))
    }
}

/// Retrieves a specific role by its ID.
/// 
/// This endpoint fetches a single role record using the provided ID.
/// 
/// # Request
/// 
/// `GET /role/{id}`
/// 
/// ## Path Parameters
/// 
/// * `id` (integer) - The unique identifier of the role
/// 
/// # Response
/// 
/// ## Success (200) - Role Found
/// ```json
/// {
///   "status": 200,
///   "data": [
///     {
///       "id": 1,
///       "name": "admin"
///     }
///   ],
///   "errorMessage": ""
/// }
/// ```
/// 
/// ## Success (200) - Role Not Found
/// ```json
/// {
///   "status": 200,
///   "data": [],
///   "errorMessage": "Not found"
/// }
/// ```
/// 
/// ## Error (500)
/// ```json
/// {
///   "status": 500,
///   "data": [],
///   "errorMessage": "Database error message"
/// }
/// ```
#[get("/role/{id}")]
pub async fn get_role_by_id(data: web::Data<DatabaseConnection>, _req: HttpRequest, id: web::Path<i32>) -> impl Responder {
    match RoleService::get_role_by_id(&data, id.into_inner()).await {
        Ok(Some(r)) => web::Json(json!({ "status": 200, "data": [RoleDto { id: r.id, name: r.name }], "errorMessage": "" })),
        Ok(None) => web::Json(json!({ "status": 200, "data": [], "errorMessage": "Not found" })),
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() }))
    }
}

/// Searches for roles by name using partial matching.
/// 
/// This endpoint performs a case-sensitive search for roles whose names contain
/// the provided search term.
/// 
/// # Request
/// 
/// `GET /role/by-name/{name}`
/// 
/// ## Path Parameters
/// 
/// * `name` (string) - The search term to match against role names
/// 
/// # Response
/// 
/// ## Success (200)
/// ```json
/// {
///   "status": 200,
///   "data": [
///     {
///       "id": 1,
///       "name": "admin"
///     },
///     {
///       "id": 3,
///       "name": "super_admin"
///     }
///   ],
///   "errorMessage": ""
/// }
/// ```
/// 
/// ## Error (500)
/// ```json
/// {
///   "status": 500,
///   "data": [],
///   "errorMessage": "Database error message"
/// }
/// ```
#[get("/role/by-name/{name}")]
pub async fn get_role_by_name(data: web::Data<DatabaseConnection>, _req: HttpRequest, name: web::Path<String>) -> impl Responder {
    match RoleService::get_role_by_name(&data, &name.into_inner()).await {
        Ok(role) => {
            let dtos: Vec<RoleDto> = role.into_iter().map(|r| RoleDto { id: r.id, name: r.name }).collect();
            web::Json(json!({ "status": 200, "data": dtos, "errorMessage": "" }))
        },
        // Ok(None) => web::Json(json!({ "status": 200, "data": [], "errorMessage": "Not found" })),
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() }))
    }
}

/// Creates a new role in the database.
/// 
/// This endpoint creates a new role with the provided name. The operation includes
/// validation to ensure no duplicate role names exist.
/// 
/// # Request
/// 
/// `POST /role`
/// 
/// ## Request Body
/// ```json
/// {
///   "name": "moderator"
/// }
/// ```
/// 
/// # Response
/// 
/// ## Success (200)
/// ```json
/// {
///   "status": 200,
///   "data": [
///     {
///       "id": 4,
///       "name": "moderator"
///     }
///   ],
///   "errorMessage": ""
/// }
/// ```
/// 
/// ## Error (500)
/// ```json
/// {
///   "status": 500,
///   "data": [],
///   "errorMessage": "role already exists"
/// }
/// ```
#[post("/role")]
pub async fn create_role(data: web::Data<DatabaseConnection>, _req: HttpRequest, payload: web::Json<CreateRoleDto>) -> impl Responder {
    match RoleService::create_role(&data, payload.name.clone()).await {
        Ok(r) => web::Json(json!({ "status": 200, "data": [RoleDto { id: r.id, name: r.name }], "errorMessage": "" })),
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() }))
    }
}

/// Updates an existing role in the database.
/// 
/// This endpoint updates the name of an existing role identified by its ID.
/// 
/// # Request
/// 
/// `PUT /role/{id}`
/// 
/// ## Path Parameters
/// 
/// * `id` (integer) - The unique identifier of the role to update
/// 
/// ## Request Body
/// ```json
/// {
///   "name": "super_admin"
/// }
/// ```
/// 
/// # Response
/// 
/// ## Success (200) - Role Updated
/// ```json
/// {
///   "status": 200,
///   "data": [
///     {
///       "id": 1,
///       "name": "super_admin"
///     }
///   ],
///   "errorMessage": ""
/// }
/// ```
/// 
/// ## Success (200) - Role Not Found
/// ```json
/// {
///   "status": 200,
///   "data": [],
///   "errorMessage": "Not found"
/// }
/// ```
/// 
/// ## Error (500)
/// ```json
/// {
///   "status": 500,
///   "data": [],
///   "errorMessage": "Database error message"
/// }
/// ```
#[put("/role/{id}")]
pub async fn update_role(data: web::Data<DatabaseConnection>, _req: HttpRequest, id: web::Path<i32>, payload: web::Json<CreateRoleDto>) -> impl Responder {
    match RoleService::update_role(&data, id.into_inner(), payload.name.clone()).await {
        Ok(Some(r)) => web::Json(json!({ "status": 200, "data": [RoleDto { id: r.id, name: r.name }], "errorMessage": "" })),
        Ok(None) => web::Json(json!({ "status": 200, "data": [], "errorMessage": "Not found" })),
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() }))
    }
}

/// Registers all role-related routes with the Actix web application.
/// 
/// This function configures the routing for all role management endpoints.
/// 
/// # Arguments
/// 
/// * `cfg` - The service configuration to register routes with
/// 
/// # Routes Registered
/// 
/// * `GET /role` - Get all roles
/// * `GET /role/{id}` - Get role by ID
/// * `GET /role/by-name/{name}` - Search roles by name
/// * `POST /role` - Create a new role
/// * `PUT /role/{id}` - Update an existing role
pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_roles);
    cfg.service(get_role_by_id);
    cfg.service(get_role_by_name);
    cfg.service(create_role);
    cfg.service(update_role);
} 