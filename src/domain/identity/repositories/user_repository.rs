use async_trait::async_trait;
use sea_orm::DbErr;
use super::super::entities::user;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<user::Model>, DbErr>;
    async fn create(&self, user: user::ActiveModel) -> Result<user::Model, DbErr>;
}