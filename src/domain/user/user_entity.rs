use sea_orm::entity::prelude::*;
use crate::domain::supplier::supplier_entity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub balance: i32,
    pub is_active: bool,
    pub role_id: i32,
    pub group_id: i32,
    pub created_date: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::super::role::role_entity::Entity",
        from = "Column::RoleId",
        to = "super::super::role::role_entity::Column::Id"
    )]
    Role,
    #[sea_orm(
        belongs_to = "super::super::group::group_entity::Entity",
        from = "Column::GroupId",
        to = "super::super::group::group_entity::Column::Id"
    )]
    Group,
    #[sea_orm(
        has_one = "supplier_entity::Entity",
        from = "Column::Id",
        to = "supplier_entity::Column::UserId"
    )]
    Supplier,
}

impl Related<super::super::role::role_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl Related<supplier_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Supplier.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
