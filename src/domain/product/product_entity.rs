use sea_orm::entity::prelude::*;
use crate::domain::group::group_entity;
use crate::domain::stock::stock_entity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "product")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub group_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "group_entity::Entity",
        from = "Column::GroupId",
        to = "group_entity::Column::Id"
    )]
    Group,
    #[sea_orm(
        has_one = "stock_entity::Entity",
        from = "Column::Id",
        to = "stock_entity::Column::ProductId"
    )]
    Stock,
}

impl Related<group_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl Related<stock_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stock.def()
    }
}

impl ActiveModelBehavior for ActiveModel {} 