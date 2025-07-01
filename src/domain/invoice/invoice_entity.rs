use sea_orm::entity::prelude::*;
use crate::domain::meal::meal_entity;
use crate::domain::group::group_entity;
use crate::domain::supplier::supplier_entity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "invoice")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub price: i64,
    pub is_deleted: bool,
    pub deleted_by: i32,
    pub created_date: DateTime,
    pub last_modification_date: DateTime,
    pub meal_id: i32,
    pub group_id: i32,
    pub supplier_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "meal_entity::Entity",
        from = "Column::MealId",
        to = "meal_entity::Column::Id"
    )]
    Meal,
    #[sea_orm(
        belongs_to = "group_entity::Entity",
        from = "Column::GroupId",
        to = "group_entity::Column::Id"
    )]
    Group,
    #[sea_orm(
        belongs_to = "supplier_entity::Entity",
        from = "Column::SupplierId",
        to = "supplier_entity::Column::Id"
    )]
    Supplier,
}

impl Related<meal_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Meal.def()
    }
}
impl Related<group_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}
impl Related<supplier_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Supplier.def()
    }
}

impl ActiveModelBehavior for ActiveModel {} 