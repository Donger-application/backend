use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct RoleDto {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct CreateRoleDto {
    #[validate(length(min = 1))]
    pub name: String,
}