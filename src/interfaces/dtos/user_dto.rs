use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserDto {
    pub id: i32,
    pub name: String,
    pub balance: i32,
    pub is_active: bool,
    pub role_id: i32,
    pub group_id: i32,
    pub created_date: chrono::NaiveDate,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
pub struct CreateUserDto {
    pub name: String,
    pub balance: i32,
    pub is_active: bool,
    pub role_id: i32,
    pub group_id: i32,
    pub created_date: chrono::NaiveDate,
}