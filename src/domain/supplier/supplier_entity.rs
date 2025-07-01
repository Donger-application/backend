use sea_orm::entity::prelude::*;
use crate::domain::user::user_entity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "supplier")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub balance: i32,
    pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user_entity::Entity",
        from = "Column::UserId",
        to = "user_entity::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<user_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {} 