use sea_orm::entity::prelude::*;
use crate::domain::order::order_entity;
use crate::domain::stock::stock_entity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "order_details")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub order_id: i32,
    pub stock_id: i32,
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
        belongs_to = "stock_entity::Entity",
        from = "Column::StockId",
        to = "stock_entity::Column::Id"
    )]
    Stock,
}

impl Related<order_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}
impl Related<stock_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stock.def()
    }
}

impl ActiveModelBehavior for ActiveModel {} 