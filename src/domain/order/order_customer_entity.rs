use sea_orm::entity::prelude::*;
use crate::domain::order::order_entity;
use crate::domain::customer::customer_entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "order_customer")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub order_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub customer_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "order_entity::Entity",
        from = "Column::OrderId",
        to = "order_entity::Column::Id"
    )]
    Order,
    #[sea_orm(
        belongs_to = "customer_entity::Entity",
        from = "Column::CustomerId",
        to = "customer_entity::Column::Id"
    )]
    Customer,
}

impl Related<order_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}
impl Related<customer_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {} 