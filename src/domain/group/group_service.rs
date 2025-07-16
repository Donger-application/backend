use crate::domain::group::group_entity::{Entity as Group};
use sea_orm::{DatabaseConnection, EntityTrait};

pub struct GroupService;

impl GroupService {
    pub async fn exists_by_id(db: &DatabaseConnection, id: i32) -> Result<bool, sea_orm::DbErr> {
        Ok(Group::find_by_id(id).one(db).await?.is_some())
    }
}
