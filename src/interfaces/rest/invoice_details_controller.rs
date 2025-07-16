use crate::domain::invoice_details::invoice_details_service::InvoiceDetailsService;
use crate::interfaces::dtos::invoice_details_dto::InvoiceDetailsDto;
use crate::interfaces::dtos::response_dto::ApiResponse;
use actix_web::{post, web, HttpRequest, Responder};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BatchCreateInvoiceDetailsDto {
    pub invoice_id: i32,
    pub product_ids: Vec<i32>,
}

#[post("/invoice-details/batch")]
pub async fn create_invoice_details_batch(
    data: web::Data<DatabaseConnection>,
    _req: HttpRequest,
    payload: web::Json<BatchCreateInvoiceDetailsDto>,
) -> impl Responder {
    match InvoiceDetailsService::create_invoice_details_with_products(
        &data,
        payload.invoice_id,
        payload.product_ids.clone(),
    )
    .await
    {
        Ok(details) => {
            let dtos: Vec<InvoiceDetailsDto> = details.into_iter().map(|d| d.into()).collect();
            web::Json(ApiResponse::new(200, dtos, ""))
        }
        Err(e) => web::Json(ApiResponse::new(500, Vec::<InvoiceDetailsDto>::new(), e.to_string())),
    }
}

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_invoice_details_batch);
} 