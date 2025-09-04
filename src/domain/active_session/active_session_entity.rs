use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ActiveSession {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32,
}