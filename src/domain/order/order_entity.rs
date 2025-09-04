use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: i32,
    pub is_deleted: bool,
    pub deleted_by: i32,
    pub created_date: chrono::NaiveDateTime,
    pub last_modification_date: chrono::NaiveDateTime,
    pub group_id: i32,
}