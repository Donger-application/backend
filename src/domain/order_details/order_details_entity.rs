use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderDetails {
    pub id: i32,
    pub order_id: i32,
    pub stock_id: i32,
}