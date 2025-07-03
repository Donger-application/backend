use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter};
use crate::domain::role::role_entity::{Entity as Role, Model as RoleModel, ActiveModel as RoleActiveModel};
use crate::domain::role::role_entity;

pub struct RoleService;

impl RoleService {
    pub async fn get_all_roles(db: &DatabaseConnection) -> Result<Vec<RoleModel>, sea_orm::DbErr> {
        Role::find().all(db).await
    }

    pub async fn get_role_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<RoleModel>, sea_orm::DbErr> {
        Role::find_by_id(id).one(db).await
    }

    pub async fn get_role_by_name(db: &DatabaseConnection, name: &str) -> Result<Vec<RoleModel>, sea_orm::DbErr> {
        Role::find().filter(role_entity::Column::Name.contains(name)).all(db).await
    }

    pub async fn create_role(db: &DatabaseConnection, name: String) -> Result<RoleModel, sea_orm::DbErr> {
        let active_model = RoleActiveModel {
            name: Set(name),
            ..Default::default()
        };
        active_model.insert(db).await
    }

    pub async fn update_role(db: &DatabaseConnection, id: i32, name: String) -> Result<Option<RoleModel>, sea_orm::DbErr> {
        if let Some(role) = Role::find_by_id(id).one(db).await? {
            let mut active_model: RoleActiveModel = role.into();
            active_model.name = Set(name);
            let updated = active_model.update(db).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }
}
