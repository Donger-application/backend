use crate::domain::invoice::invoice_entity::{ Entity as Invoice, Model as InvoiceModel};
use crate::domain::meal::meal_entity::{Entity as Meal};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};

pub struct InvoiceService;

impl InvoiceService {
    pub async fn get_invoices(db: &DatabaseConnection, group_id: i32) -> Result<Vec<InvoiceModel>, sea_orm::DbErr> {
        Invoice::find().filter(crate::domain::invoice::invoice_entity::Column::GroupId.eq(group_id)).all(db).await
    }

    pub async fn get_invoice_by_id(db: &DatabaseConnection, id: i32, group_id: i32) -> Result<Option<InvoiceModel>, sea_orm::DbErr> {
        Invoice::find().filter(crate::domain::invoice::invoice_entity::Column::Id.eq(id)).filter(crate::domain::invoice::invoice_entity::Column::GroupId.eq(group_id)).one(db).await
    }

    pub async fn get_invoices_by_meal_id(db: &DatabaseConnection, meal_id: i32, group_id: i32) -> Result<Vec<InvoiceModel>, sea_orm::DbErr> {
        Invoice::find()
            .filter(crate::domain::invoice::invoice_entity::Column::MealId.eq(meal_id))
            .filter(crate::domain::invoice::invoice_entity::Column::GroupId.eq(group_id))
            .all(db)
            .await
    }

    pub async fn get_invoices_by_supplier_id(db: &DatabaseConnection, supplier_id: i32, group_id: i32) -> Result<Vec<InvoiceModel>, sea_orm::DbErr> {
        Invoice::find()
            .filter(crate::domain::invoice::invoice_entity::Column::SupplierId.eq(supplier_id))
            .filter(crate::domain::invoice::invoice_entity::Column::GroupId.eq(group_id))
            .all(db)
            .await
    }

    pub async fn get_invoices_by_product_id(db: &DatabaseConnection, product_id: i32, group_id: i32) -> Result<Vec<InvoiceModel>, sea_orm::DbErr> {
        // Find all meals with the given product_id
        let meal_ids: Vec<i32> = Meal::find()
            .filter(crate::domain::meal::meal_entity::Column::ProductId.eq(product_id))
            .all(db)
            .await?
            .into_iter()
            .map(|meal| meal.id)
            .collect();
        if meal_ids.is_empty() {
            return Ok(vec![]);
        }
        Invoice::find()
            .filter(crate::domain::invoice::invoice_entity::Column::GroupId.eq(group_id))
            .filter(crate::domain::invoice::invoice_entity::Column::MealId.is_in(meal_ids))
            .all(db)
            .await
    }
} 