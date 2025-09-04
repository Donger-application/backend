
use actix_web::{web};

// #[post("/invoice-details/batch")]
// pub async fn create_invoice_details_batch(
//     data: web::Data<DatabaseConnection>,
//     _req: HttpRequest,
//     payload: web::Json<BatchCreateInvoiceDetailsDto>,
// ) -> impl Responder {
//     match InvoiceDetailsService::create_invoice_details_with_products(
//         &data,
//         payload.invoice_id,
//         payload.product_ids.clone(),
//     )
//     .await
//     {
//         Ok(details) => {
//             let dtos: Vec<InvoiceDetailsDto> = details.into_iter().map(|d| d.into()).collect();
//             web::Json(ApiResponse::new(200, dtos, ""))
//         }
//         Err(e) => web::Json(ApiResponse::new(500, Vec::<InvoiceDetailsDto>::new(), e.to_string())),
//     }
// }

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    // cfg.service(create_invoice_details_batch);
} 