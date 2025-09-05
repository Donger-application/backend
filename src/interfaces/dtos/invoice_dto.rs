use sqlx::FromRow;
use chrono::NaiveDateTime;
use crate::interfaces::dtos::supplier_dto::SupplierDto;

#[derive(Debug, FromRow)]
struct InvoiceRow {
    id: i32,
    price: i32,
    created_date: NaiveDateTime,
    supplier_id: i32,
    supplier_name: String,
    meal: String, // string_agg result
}

#[derive(Debug)]
pub struct InvoiceDto {
    supplier: SupplierDto,
    price: i32,
    meal: Vec<String>, // split string_agg into Vec<String>
}

#[derive(Debug)]
struct InvoiceResponse {
    date: NaiveDateTime,
    invoices: Vec<InvoiceDto>,
}
