use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Stock {
    pub id: i32,
    pub price: i64,
    pub consumed: bool,
    pub product_id: i32,
}