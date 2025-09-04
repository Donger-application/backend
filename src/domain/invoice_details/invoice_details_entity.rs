use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InvoiceDetails {
    pub id: i32,
    pub invoice_id: i32,
    pub stock_id: i32,
}