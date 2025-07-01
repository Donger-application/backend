use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct UserDto {
    pub id: i32,
    pub name: String,
    pub role_id: i32,
    pub group_id: i32,
    pub balance: i32,
    pub is_active: bool,
    pub created_date: DateTime<Utc>,
}

#[derive(Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 1))]
    pub name: String,
    pub role_id: i32,
    pub group_id: i32,
}