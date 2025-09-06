use crate::{
    domain::invoice::invoice_service::InvoiceService,
    interfaces::dtos::{invoice_dto::{InvoiceFilter, InvoiceResponse}, response_dto::ApiResponse},
};
use actix_web::{get, web, HttpRequest, Responder};
use sqlx::PgPool;

#[get("/invoice/group/{group_id}")]
pub async fn get_invoice_report(
    data: web::Data<PgPool>,
    _req: HttpRequest,
    group_id: web::Path<i32>,
    filter: web::Json<InvoiceFilter>,
) -> impl Responder {
    match InvoiceService::get_invoice_report(
        group_id.into_inner(),
        filter.start_date.clone(),
        filter.end_date.clone(),
        data.get_ref().clone(),
    )
    .await
    {
        Ok(invoices) => {
            web::Json(ApiResponse::new(200, invoices, ""))
        }
        Err(e) => web::Json(ApiResponse::new(
            500,
            Vec::<InvoiceResponse>::new(),
            e.to_string(),
        )),
    }
}

// #[get("/invoice/{id}/group/{group_id}")]
// pub async fn get_invoice_by_id(
//     data: web::Data<PgPool>,
//     _req: HttpRequest,
//     path: web::Path<(i32, i32)>,
// ) -> impl Responder {
//     let (id, group_id) = path.into_inner();
//     match InvoiceService::get_invoice_by_id(&data, id, group_id).await {
//         Ok(Some(inv)) => {
//             let dto: InvoiceDto = inv.into();
//             web::Json(ApiResponse::new(200, vec![dto], ""))
//         }
//         Ok(None) => web::Json(ApiResponse::new(404, Vec::<InvoiceDto>::new(), "Not found")),
//         Err(e) => web::Json(ApiResponse::new(500, Vec::<InvoiceDto>::new(), e.to_string())),
//     }
// }

// #[get("/invoice/meal/{meal_id}/group/{group_id}")]
// pub async fn get_invoices_by_meal_id(
//     data: web::Data<PgPool>,
//     _req: HttpRequest,
//     path: web::Path<(i32, i32)>,
// ) -> impl Responder {
//     let (meal_id, group_id) = path.into_inner();
//     match InvoiceService::get_invoices_by_meal_id(&data, meal_id, group_id).await {
//         Ok(invoices) => {
//             let dtos: Vec<InvoiceDto> = invoices.into_iter().map(|inv| inv.into()).collect();
//             web::Json(ApiResponse::new(200, dtos, ""))
//         }
//         Err(e) => web::Json(ApiResponse::new(500, Vec::<InvoiceDto>::new(), e.to_string())),
//     }
// }

// #[get("/invoice/supplier/{supplier_id}/group/{group_id}")]
// pub async fn get_invoices_by_supplier_id(
//     data: web::Data<PgPool>,
//     _req: HttpRequest,
//     path: web::Path<(i32, i32)>,
// ) -> impl Responder {
//     let (supplier_id, group_id) = path.into_inner();
//     match InvoiceService::get_invoices_by_supplier_id(&data, supplier_id, group_id).await {
//         Ok(invoices) => {
//             let dtos: Vec<InvoiceDto> = invoices.into_iter().map(|inv| inv.into()).collect();
//             web::Json(ApiResponse::new(200, dtos, ""))
//         }
//         Err(e) => web::Json(ApiResponse::new(500, Vec::<InvoiceDto>::new(), e.to_string())),
//     }
// }

// #[get("/invoice/product/{product_id}/group/{group_id}")]
// pub async fn get_invoices_by_product_id(
//     data: web::Data<PgPool>,
//     _req: HttpRequest,
//     path: web::Path<(i32, i32)>,
// ) -> impl Responder {
//     let (product_id, group_id) = path.into_inner();
//     match InvoiceService::get_invoices_by_product_id(&data, product_id, group_id).await {
//         Ok(invoices) => {
//             let dtos: Vec<InvoiceDto> = invoices.into_iter().map(|inv| inv.into()).collect();
//             web::Json(ApiResponse::new(200, dtos, ""))
//         }
//         Err(e) => web::Json(ApiResponse::new(500, Vec::<InvoiceDto>::new(), e.to_string())),
//     }
// }

// #[get("/invoice/weekly")]
// pub async fn get_invoices_weekly(data: web::Data<PgPool>, _req:HttpRequest, group_id: web::Path<i32>)-> impl Responder{
//     match InvoiceService::get_invoices_weekly(&data, group_id.into_inner()).await {
//         Ok(invoices) => {
//             let dtos: Vec<InvoiceDto> = invoices.into_iter().map(|inv| inv.into()).collect();
//             web::Json(ApiResponse::new(200, dtos, ""))
//         }
//         Err(e) => web::Json(ApiResponse::new(500, Vec::<InvoiceDto>::new(), e.to_string())),
//     }
// }

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_invoice_report);
    // cfg.service(get_invoice_by_id);
    // cfg.service(get_invoices_by_meal_id);
    // cfg.service(get_invoices_by_supplier_id);
    // cfg.service(get_invoices_by_product_id);
}
