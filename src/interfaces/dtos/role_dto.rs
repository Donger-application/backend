use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RoleDto {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
pub struct CreateRoleDto {
    #[validate(length(min = 1))]
    pub name: String,
}