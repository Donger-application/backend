use sea_orm::{DatabaseConnection, DbErr, ActiveValue};
use super::super::entities::user;
use super::super::repositories::user_repository::UserRepository;

pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {
    pub fn new(db: &DatabaseConnection) -> Self {
        UserService { db: db.clone() }
    }

    pub async fn get_user_data(&self, user_id: i32) -> Result<user::Model, DbErr> {
        user::Entity::find_by_id(user_id)
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".into()))
    }

    pub async fn create_user(&self, name: String, role_id: i32) -> Result<user::Model, DbErr> {
        let user = user::ActiveModel {
            name: ActiveValue::Set(name),
            role_id: ActiveValue::Set(role_id),
            is_active: ActiveValue::Set(true),
            register_date: ActiveValue::Set(chrono::Utc::now()),
            ..Default::default()
        };
        user.insert(&self.db).await
    }
}