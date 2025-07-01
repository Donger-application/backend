use sea_orm::entity::prelude::*;
use crate::domain::group::group_entity;
use crate::domain::customer::customer_entity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "order")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub is_deleted: bool,
    pub deleted_by: i32,
    pub created_date: DateTime,
    pub last_modification_date: DateTime,
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
        via = "super::order_customer_entity::Entity",
        from = "Column::Id",
        to = "super::order_customer_entity::Column::OrderId"
    )]
    OrderCustomer,
    #[sea_orm(
        belongs_to = "customer_entity::Entity",
        from = "Column::Id",
        to = "customer_entity::Column::Id"
    )]
    Customer,
}

impl Related<group_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl ActiveModelBehavior for ActiveModel {} 