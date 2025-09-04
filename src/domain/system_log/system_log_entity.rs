use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Model {
    pub id: i32,
    pub transaction_type: String,
    pub description: String,
    pub date: chrono::NaiveDateTime,
    pub user_id: i32,
    pub group_id: i32,
}