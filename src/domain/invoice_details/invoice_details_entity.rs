use sea_orm::entity::prelude::*;
use crate::domain::invoice::invoice_entity;
use crate::domain::stock::stock_entity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "invoice_details")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub invoice_id: i32,
    pub stock_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "invoice_entity::Entity",
        from = "Column::InvoiceId",
        to = "invoice_entity::Column::Id"
    )]
    Invoice,
    #[sea_orm(
        belongs_to = "stock_entity::Entity",
        from = "Column::StockId",
        to = "stock_entity::Column::Id"
    )]
    Stock,
}

impl Related<invoice_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Invoice.def()
    }
}
impl Related<stock_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stock.def()
    }
}

impl ActiveModelBehavior for ActiveModel {} 