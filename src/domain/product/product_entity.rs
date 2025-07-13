use crate::domain::group::group_entity;
use crate::domain::stock::stock_entity;
use crate::interfaces::dtos::product_dto::ProductDto;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "product")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub group_id: i32,
}

impl From<Model> for ProductDto {
    fn from(product: Model) -> Self {
        ProductDto {
            id: product.id,
            name: product.name,
            group_id: product.group_id
        }
    }
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
