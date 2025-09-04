use actix_web::{web};

// #[get("/product")]
// pub async fn get_all_products(
//     data: web::Data<DatabaseConnection>,
//     _req: HttpRequest,
// ) -> impl Responder {
//     match ProductService::get_all_products(&data).await {
//         Ok(product) => {
//             let dtos: Vec<ProductDto> = product.into_iter().map(|product| product.into()).collect();
//             web::Json(json!({ "status": 200, "data": dtos, "error_message": "" }))
//         }
//         Err(e) => web::Json(json!({ "status": 500, "data": [], "error_message": e.to_string() })),
//     }
// }


pub fn register_routes(cfg: &mut web::ServiceConfig) {
    // cfg.service(get_all_products);
}