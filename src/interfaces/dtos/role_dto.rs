use utoipa::ToSchema;
use validator::Validate;
use crate::domain::role::role_entity;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct RoleDto {
    pub id: i32,
    pub name: String,
}

impl From<role_entity::Model> for RoleDto {
    fn from(model: role_entity::Model) -> Self {
        RoleDto {
            id: model.id,
            name: model.name,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate, ToSchema)]
pub struct CreateRoleDto {
    #[validate(length(min = 1))]
    pub name: String,
}