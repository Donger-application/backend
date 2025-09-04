use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Invoice {
    pub id: i32,
    pub price: i64,
    pub is_deleted: bool,
    pub deleted_by: i32,
    pub created_date: chrono::NaiveDateTime,
    pub last_modification_date: chrono::NaiveDateTime,
    pub meal_id: i32,
    pub group_id: i32,
    pub supplier_id: i32,
}