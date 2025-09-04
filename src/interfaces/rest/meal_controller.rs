use crate::domain::meal::meal_service::MealService;
use crate::interfaces::dtos::meal_dto::{CreateMealDto, MealDto};
use crate::interfaces::dtos::response_dto::ApiResponse;
use actix_web::{delete, get, post, put, web, HttpRequest, Responder};
use sea_orm::DatabaseConnection;

#[get("/meal")]
pub async fn get_all_meals(
    data: web::Data<DatabaseConnection>,
    _req: HttpRequest,
) -> impl Responder {
    match MealService::get_all_meals(&data).await {
        Ok(meals) => {
            let dtos: Vec<MealDto> = meals.into_iter().map(|meal| MealDto { id: meal.id, product_id: meal.product_id }).collect();
            web::Json(ApiResponse::new(200, dtos, ""))
        }
        Err(e) => web::Json(ApiResponse::new(500, Vec::<MealDto>::new(), e.to_string())),
    }
}

#[get("/meal/{id}")]
pub async fn get_meal_by_id(
    data: web::Data<DatabaseConnection>,
    _req: HttpRequest,
    id: web::Path<i32>,
) -> impl Responder {
    match MealService::get_meal_by_id(&data, id.into_inner()).await {
        Ok(Some(meal)) => {
            let dto = MealDto { id: meal.id, product_id: meal.product_id };
            web::Json(ApiResponse::new(200, vec![dto], ""))
        }
        Ok(None) => web::Json(ApiResponse::new(404, Vec::<MealDto>::new(), "Not found")),
        Err(e) => web::Json(ApiResponse::new(500, Vec::<MealDto>::new(), e.to_string())),
    }
}

#[get("/meal/product/{product_id}")]
pub async fn get_meals_by_product_id(
    data: web::Data<DatabaseConnection>,
    _req: HttpRequest,
    product_id: web::Path<i32>,
) -> impl Responder {
    match MealService::get_meals_by_product_id(&data, product_id.into_inner()).await {
        Ok(meals) => {
            let dtos: Vec<MealDto> = meals.into_iter().map(|meal| MealDto { id: meal.id, product_id: meal.product_id }).collect();
            web::Json(ApiResponse::new(200, dtos, ""))
        }
        Err(e) => web::Json(ApiResponse::new(500, Vec::<MealDto>::new(), e.to_string())),
    }
}

#[post("/meal")]
pub async fn create_meal(
    data: web::Data<DatabaseConnection>,
    _req: HttpRequest,
    payload: web::Json<CreateMealDto>,
) -> impl Responder {
    match MealService::create_meal(&data, payload.product_id).await {
        Ok(meal) => {
            let dto = MealDto { id: meal.id, product_id: meal.product_id };
            web::Json(ApiResponse::new(200, vec![dto], ""))
        }
        Err(e) => web::Json(ApiResponse::new(500, Vec::<MealDto>::new(), e.to_string())),
    }
}

#[put("/meal/{id}")]
pub async fn update_meal(
    data: web::Data<DatabaseConnection>,
    _req: HttpRequest,
    id: web::Path<i32>,
    payload: web::Json<CreateMealDto>,
) -> impl Responder {
    match MealService::update_meal(&data, id.into_inner(), Some(payload.product_id)).await {
        Ok(Some(meal)) => {
            let dto = MealDto { id: meal.id, product_id: meal.product_id };
            web::Json(ApiResponse::new(200, vec![dto], ""))
        }
        Ok(None) => web::Json(ApiResponse::new(404, Vec::<MealDto>::new(), "Not found")),
        Err(e) => web::Json(ApiResponse::new(500, Vec::<MealDto>::new(), e.to_string())),
    }
}

#[delete("/meal/{id}")]
pub async fn delete_meal(
    data: web::Data<DatabaseConnection>,
    _req: HttpRequest,
    id: web::Path<i32>,
) -> impl Responder {
    match MealService::delete_meal(&data, id.into_inner()).await {
        Ok(true) => web::Json(ApiResponse::new(200, Vec::<MealDto>::new(), "")),
        Ok(false) => web::Json(ApiResponse::new(404, Vec::<MealDto>::new(), "Not found")),
        Err(e) => web::Json(ApiResponse::new(500, Vec::<MealDto>::new(), e.to_string())),
    }
}

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_meals);
    cfg.service(get_meal_by_id);
    cfg.service(get_meals_by_product_id);
    cfg.service(create_meal);
    cfg.service(update_meal);
    cfg.service(delete_meal);
} 