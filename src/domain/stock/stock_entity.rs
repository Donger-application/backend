use sea_orm::entity::prelude::*;
use crate::domain::product::product_entity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "stock")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub price: i64,
    pub consumed: bool,
    pub product_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "product_entity::Entity",
        from = "Column::ProductId",
        to = "product_entity::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Product,
}

impl Related<product_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl ActiveModelBehavior for ActiveModel {} 