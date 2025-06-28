use sea_orm::{DatabaseConnection, DbErr, ActiveValue};
use super::super::entities::role_entity;
use super::super::repositories::role_repository::RoleRepository;

pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {
    pub fn new(db: &DatabaseConnection) -> Self {
        UserService { db: db.clone() }
    }

    pub async fn get_role_data(&self, role_id: i32) -> Result<role_entity::Model, DbErr> {
        role_entity::Entity::find_by_id(role_id)
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("Role not found".into()))
    }

    pub async fn create_role(&self, name: String) -> Result<role_entity::Model, DbErr> {
        let role = role_entity::ActiveModel {
            name: ActiveValue::Set(name),
            ..Default::default()
        };
        role.insert(&self.db).await
    }
}