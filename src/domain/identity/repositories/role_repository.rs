use async_trait::async_trait;
use sea_orm::DbErr;
use super::super::entities::role;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<role::Model>, DbErr>;
    async fn create(&self, role: role::ActiveModel) -> Result<role::Model, DbErr>;
}