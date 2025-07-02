use sea_orm::entity::prelude::*;
use crate::domain::user::user_entity;
use crate::domain::group::group_entity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "system_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub transaction_type: String,
    pub description: String,
    pub date: DateTime,
    pub user_id: i32,
    pub group_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user_entity::Entity",
        from = "Column::UserId",
        to = "user_entity::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "group_entity::Entity",
        from = "Column::GroupId",
        to = "group_entity::Column::Id"
    )]
    Group,
}

impl Related<user_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
impl Related<group_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl ActiveModelBehavior for ActiveModel {} 