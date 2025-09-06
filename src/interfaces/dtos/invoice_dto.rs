use std::collections::HashMap;

use crate::interfaces::dtos::supplier_dto::SupplierDto;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InvoiceRow {
    pub id: i32,
    pub price: i64,
    pub created_date: NaiveDateTime,
    pub supplier_id: i32,
    pub supplier_name: String,
    pub meal: String,
}

#[derive(Debug, Deserialize)]
pub struct InvoiceFilter {
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceDto {
    pub supplier: SupplierDto,
    pub price: i64,
    pub meal: String,
    pub invoice_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceResponse {
    pub date: String,
    pub invoices: Vec<InvoiceDto>,
}

impl From<InvoiceRow> for InvoiceDto {
    fn from(row: InvoiceRow) -> Self {
        InvoiceDto {
            invoice_id: row.id,
            price: row.price,
            meal: row.meal,
            supplier: SupplierDto {
                id: row.supplier_id,
                name: row.supplier_name,
            },
        }
    }
}

pub fn group_invoices(rows: Vec<InvoiceRow>) -> Vec<InvoiceResponse> {
    let mut map: HashMap<NaiveDate, Vec<InvoiceDto>> = HashMap::new();

    for row in rows {
        map.entry(row.created_date.date())
            .or_default()
            .push(row.into());
    }

    let mut result: Vec<InvoiceResponse> = map
        .into_iter()
        .map(|(date, invoices)| InvoiceResponse {
            date: date.format("%Y-%m-%d").to_string(),
            invoices,
        })
        .collect();

    result.sort_by_key(|resp| resp.date.clone());

    result
}