use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {

    pub id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
    pub email_confirmed: bool,
    pub user_display_id: String,
    pub balance: i32,
    pub is_active: bool,
    pub role_id: i32,
    pub group_id: i32,
    pub created_date: chrono::NaiveDateTime,
}