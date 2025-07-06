use actix_web::{get, post, put, delete, web, HttpRequest, Responder, HttpResponse};
use serde_json::json;
use crate::domain::user::user_service::UserService;
use crate::interfaces::dtos::user_dto::{UserDto, CreateUserDto};
use sea_orm::DatabaseConnection;

/// REST API endpoints for user management.
/// 
/// This module provides HTTP endpoints for performing CRUD operations on users.
/// All endpoints return JSON responses with a consistent format:
/// ```json
/// {
///   "status": 200,
///   "data": [...],
///   "errorMessage": ""
/// }
/// ```
/// 
/// # Features
/// 
/// * Complete user CRUD operations
/// * Search functionality by name, email, and display ID
/// * Comprehensive validation and error handling
/// * Automatic mapping between entities and DTOs
/// 
/// # Endpoints
/// 
/// * `GET /users` - Get all users
/// * `GET /users/{id}` - Get user by ID
/// * `GET /users/email/{email}` - Get user by email
/// * `GET /users/display/{display_id}` - Get user by display ID
/// * `GET /users/search/{name}` - Search users by name
/// * `POST /users` - Create a new user
/// * `PUT /users/{id}` - Update an existing user
/// * `DELETE /users/{id}` - Delete a user

/// Retrieves all users from the database.
/// 
/// This endpoint fetches all user records and returns them as a JSON array.
/// 
/// # Request
/// 
/// `GET /users`
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
///       "name": "John Doe",
///       "password": "hashed_password",
///       "email": "john@example.com",
///       "email_confirmed": false,
///       "user_display_id": "john_doe",
///       "balance": 0,
///       "is_active": true,
///       "role_id": 1,
///       "group_id": 1,
///       "created_date": "2024-01-01"
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

/// Retrieves a specific user by their ID.
/// 
/// This endpoint fetches a single user record using the provided ID.
/// 
/// # Request
/// 
/// `GET /users/{id}`
/// 
/// ## Path Parameters
/// 
/// * `id` (integer) - The unique identifier of the user
/// 
/// # Response
/// 
/// ## Success (200) - User Found
/// ```json
/// {
///   "status": 200,
///   "data": [
///     {
///       "id": 1,
///       "name": "John Doe",
///       "password": "hashed_password",
///       "email": "john@example.com",
///       "email_confirmed": false,
///       "user_display_id": "john_doe",
///       "balance": 0,
///       "is_active": true,
///       "role_id": 1,
///       "group_id": 1,
///       "created_date": "2024-01-01"
///     }
///   ],
///   "errorMessage": ""
/// }
/// ```
/// 
/// ## Success (200) - User Not Found
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

/// Retrieves a specific user by their email address.
/// 
/// This endpoint fetches a single user record using the provided email address.
/// Since email addresses are unique, this endpoint returns at most one user.
/// 
/// # Request
/// 
/// `GET /users/email/{email}`
/// 
/// ## Path Parameters
/// 
/// * `email` (string) - The email address of the user to retrieve
/// 
/// # Response
/// 
/// ## Success (200) - User Found
/// ```json
/// {
///   "status": 200,
///   "data": [
///     {
///       "id": 1,
///       "name": "John Doe",
///       "password": "hashed_password",
///       "email": "john@example.com",
///       "email_confirmed": false,
///       "user_display_id": "john_doe",
///       "balance": 0,
///       "is_active": true,
///       "role_id": 1,
///       "group_id": 1,
///       "created_date": "2024-01-01"
///     }
///   ],
///   "errorMessage": ""
/// }
/// ```
/// 
/// ## Success (200) - User Not Found
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

/// Retrieves a specific user by their display ID.
/// 
/// This endpoint fetches a single user record using the provided display ID.
/// Since display IDs are unique, this endpoint returns at most one user.
/// 
/// # Request
/// 
/// `GET /users/display/{display_id}`
/// 
/// ## Path Parameters
/// 
/// * `display_id` (string) - The display ID of the user to retrieve
/// 
/// # Response
/// 
/// ## Success (200) - User Found
/// ```json
/// {
///   "status": 200,
///   "data": [
///     {
///       "id": 1,
///       "name": "John Doe",
///       "password": "hashed_password",
///       "email": "john@example.com",
///       "email_confirmed": false,
///       "user_display_id": "john_doe",
///       "balance": 0,
///       "is_active": true,
///       "role_id": 1,
///       "group_id": 1,
///       "created_date": "2024-01-01"
///     }
///   ],
///   "errorMessage": ""
/// }
/// ```
/// 
/// ## Success (200) - User Not Found
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

/// Retrieves users by name with partial matching.
/// 
/// This endpoint fetches user records that match the provided name pattern.
/// The search is case-insensitive and supports partial matching.
/// 
/// # Request
/// 
/// `GET /users/search/{name}`
/// 
/// ## Path Parameters
/// 
/// * `name` (string) - The name pattern to search for (supports partial matching)
/// 
/// # Response
/// 
/// ## Success (200) - Users Found
/// ```json
/// {
///   "status": 200,
///   "data": [
///     {
///       "id": 1,
///       "name": "John Doe",
///       "password": "hashed_password",
///       "email": "john@example.com",
///       "email_confirmed": false,
///       "user_display_id": "john_doe",
///       "balance": 0,
///       "is_active": true,
///       "role_id": 1,
///       "group_id": 1,
///       "created_date": "2024-01-01"
///     },
///     {
///       "id": 2,
///       "name": "Johnny Smith",
///       "password": "hashed_password",
///       "email": "johnny@example.com",
///       "email_confirmed": true,
///       "user_display_id": "johnny_smith",
///       "balance": 100,
///       "is_active": true,
///       "role_id": 2,
///       "group_id": 1,
///       "created_date": "2024-01-02"
///     }
///   ],
///   "errorMessage": ""
/// }
/// ```
/// 
/// ## Success (200) - No Users Found
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

/// Creates a new user in the system.
/// 
/// This endpoint creates a new user record with the provided information.
/// The system validates that the email and user_display_id are unique,
/// and that the specified role and group exist.
/// 
/// # Request
/// 
/// `POST /users`
/// 
/// ## Request Body
/// ```json
/// {
///   "name": "John Doe",
///   "password": "secure_password",
///   "email": "john@example.com",
///   "email_confirmed": false,
///   "user_display_id": "john_doe",
///   "balance": 0,
///   "is_active": true,
///   "role_id": 1,
///   "group_id": 1,
///   "created_date": "2024-01-01"
/// }
/// ```
/// 
/// ## Validation Rules
/// 
/// * `name` - Required, non-empty string
/// * `password` - Required, non-empty string
/// * `email` - Required, valid email format, must be unique
/// * `user_display_id` - Required, non-empty string, must be unique
/// * `role_id` - Required, must reference an existing role
/// * `group_id` - Required, must reference an existing group
/// 
/// # Response
/// 
/// ## Success (200) - User Created
/// ```json
/// {
///   "status": 200,
///   "data": [
///     {
///       "id": 1,
///       "name": "John Doe",
///       "password": "hashed_password",
///       "email": "john@example.com",
///       "email_confirmed": false,
///       "user_display_id": "john_doe",
///       "balance": 0,
///       "is_active": true,
///       "role_id": 1,
///       "group_id": 1,
///       "created_date": "2024-01-01"
///     }
///   ],
///   "errorMessage": ""
/// }
/// ```
/// 
/// ## Error (500) - Validation Error
/// ```json
/// {
///   "status": 500,
///   "data": [],
///   "errorMessage": "Email already exists"
/// }
/// ```
/// 
/// ## Error (500) - Database Error
/// ```json
/// {
///   "status": 500,
///   "data": [],
///   "errorMessage": "Database error message"
/// }
/// ```
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

/// Updates an existing user in the system.
/// 
/// This endpoint updates a user record with the provided information.
/// The system validates that the email and user_display_id are unique
/// (only if they are being changed), and that the specified role and group exist.
/// 
/// # Request
/// 
/// `PUT /users/{id}`
/// 
/// ## Path Parameters
/// 
/// * `id` (integer) - The unique identifier of the user to update
/// 
/// ## Request Body
/// ```json
/// {
///   "name": "John Doe Updated",
///   "password": "new_secure_password",
///   "email": "john.updated@example.com",
///   "email_confirmed": true,
///   "user_display_id": "john_doe_updated",
///   "balance": 100,
///   "is_active": true,
///   "role_id": 2,
///   "group_id": 1,
///   "created_date": "2024-01-01"
/// }
/// ```
/// 
/// ## Validation Rules
/// 
/// * `name` - Required, non-empty string
/// * `password` - Required, non-empty string
/// * `email` - Required, valid email format, must be unique (if changed)
/// * `user_display_id` - Required, non-empty string, must be unique (if changed)
/// * `role_id` - Required, must reference an existing role
/// * `group_id` - Required, must reference an existing group
/// 
/// # Response
/// 
/// ## Success (200) - User Updated
/// ```json
/// {
///   "status": 200,
///   "data": [
///     {
///       "id": 1,
///       "name": "John Doe Updated",
///       "password": "hashed_password",
///       "email": "john.updated@example.com",
///       "email_confirmed": true,
///       "user_display_id": "john_doe_updated",
///       "balance": 100,
///       "is_active": true,
///       "role_id": 2,
///       "group_id": 1,
///       "created_date": "2024-01-01"
///     }
///   ],
///   "errorMessage": ""
/// }
/// ```
/// 
/// ## Success (200) - User Not Found
/// ```json
/// {
///   "status": 200,
///   "data": [],
///   "errorMessage": "Not found"
/// }
/// ```
/// 
/// ## Error (500) - Validation Error
/// ```json
/// {
///   "status": 500,
///   "data": [],
///   "errorMessage": "Email already exists"
/// }
/// ```
/// 
/// ## Error (500) - Database Error
/// ```json
/// {
///   "status": 500,
///   "data": [],
///   "errorMessage": "Database error message"
/// }
/// ```
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

/// Deletes a user from the system.
/// 
/// This endpoint permanently removes a user record from the database.
/// The operation is irreversible and will delete all associated user data.
/// 
/// # Request
/// 
/// `DELETE /users/{id}`
/// 
/// ## Path Parameters
/// 
/// * `id` (integer) - The unique identifier of the user to delete
/// 
/// # Response
/// 
/// ## Success (200) - User Deleted
/// ```json
/// {
///   "status": 200,
///   "data": [],
///   "errorMessage": ""
/// }
/// ```
/// 
/// ## Success (200) - User Not Found
/// ```json
/// {
///   "status": 200,
///   "data": [],
///   "errorMessage": "Not found"
/// }
/// ```
/// 
/// ## Error (500) - Database Error
/// ```json
/// {
///   "status": 500,
///   "data": [],
///   "errorMessage": "Database error message"
/// }
/// ```
/// 
/// # Notes
/// 
/// * This operation is permanent and cannot be undone
/// * Any foreign key constraints will prevent deletion if the user is referenced elsewhere
/// * Consider deactivating users instead of deleting them for audit purposes
#[delete("/users/{id}")]
pub async fn delete_user(data: web::Data<DatabaseConnection>, _req: HttpRequest, id: web::Path<i32>) -> impl Responder {
    match UserService::delete_user(&data, id.into_inner()).await {
        Ok(true) => web::Json(json!({ "status": 200, "data": [], "errorMessage": "" })),
        Ok(false) => web::Json(json!({ "status": 200, "data": [], "errorMessage": "Not found" })),
        Err(e) => web::Json(json!({ "status": 500, "data": [], "errorMessage": e.to_string() }))
    }
}

/// Registers all user-related routes with the Actix web application.
/// 
/// This function configures the routing for all user management endpoints.
/// 
/// # Arguments
/// 
/// * `cfg` - The service configuration to register routes with
/// 
/// # Routes Registered
/// 
/// * `GET /users` - Get all users
/// * `GET /users/{id}` - Get user by ID
/// * `GET /users/email/{email}` - Get user by email
/// * `GET /users/display/{display_id}` - Get user by display ID
/// * `GET /users/search/{name}` - Search users by name
/// * `POST /users` - Create a new user
/// * `PUT /users/{id}` - Update an existing user
/// * `DELETE /users/{id}` - Delete a user
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