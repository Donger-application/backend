use crate::domain::group::group_service::GroupService;
use crate::domain::supplier::supplier_service::SupplierService;
use crate::domain::meal::meal_service::MealService;
use crate::domain::meal::meal_entity::{ActiveModel as MealActiveModel, Entity as Meal};
use crate::domain::invoice::invoice_entity::{ActiveModel as InvoiceActiveModel, Entity as Invoice, Model as InvoiceModel};
use crate::interfaces::rest::invoice_controller::get_invoices_weekly;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set};
use chrono::Utc;

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

    pub async fn create_invoice_with_details(
        db: &DatabaseConnection,
        price: i64,
        group_id: i32,
        supplier_id: i32,
        product_ids: Vec<i32>,
    ) -> Result<InvoiceModel, sea_orm::DbErr> {
        // 2. check if the group_id exists
        if !GroupService::exists_by_id(db, group_id).await? {
            return Err(sea_orm::DbErr::Custom("group does not exist".to_string()));
        }
        // 3. check if supplier_id exists
        if !SupplierService::exists_by_id(db, supplier_id).await? {
            return Err(sea_orm::DbErr::Custom("supplier does not exist".to_string()));
        }
        // 4. check for existing meal with product_ids
        let meal_id = if MealService::exists_with_exact_product_ids(db, &product_ids).await? {
            // Find the meal id with exactly these product_ids
            let meals = crate::domain::meal::meal_entity::Entity::find()
                .filter(crate::domain::meal::meal_entity::Column::ProductId.is_in(product_ids.clone()))
                .all(db)
                .await?;
            use std::collections::HashMap;
            let mut meal_to_products: HashMap<i32, Vec<i32>> = HashMap::new();
            for meal in meals {
                meal_to_products.entry(meal.id).or_default().push(meal.product_id);
            }
            let mut found_meal_id = None;
            let mut sorted_input = product_ids.clone();
            sorted_input.sort();
            for (meal_id, products) in meal_to_products {
                let mut sorted_db = products.clone();
                sorted_db.sort();
                if sorted_db == sorted_input {
                    found_meal_id = Some(meal_id);
                    break;
                }
            }
            found_meal_id.ok_or_else(|| sea_orm::DbErr::Custom("meal not found after check".to_string()))?
        } else {
            // 4.1. create meals using the new service function
            let created_meals = MealService::create_meal_with_products(db, &product_ids).await?;
            *created_meals.first().map(|m| &m.id).ok_or_else(|| sea_orm::DbErr::Custom("no meal created".to_string()))?
        };
        // 5. create the invoice
        let now = Utc::now().naive_utc();
        let invoice_active = InvoiceActiveModel {
            price: Set(price),
            is_deleted: Set(false),
            deleted_by: Set(0),
            created_date: Set(now),
            last_modification_date: Set(now),
            meal_id: Set(meal_id),
            group_id: Set(group_id),
            supplier_id: Set(supplier_id),
            ..Default::default()
        };
        let invoice = invoice_active.insert(db).await?;
        Ok(invoice)
    }

    pub async fn get_invoices_weekly(db: &DatabaseConnection, group_id: i32) -> Result<Vec<InvoiceModel>, sea_orm::DbErr> {
        let now = Utc::now().naive_utc();

        let invoices = Invoice::find()
            .filter(crate::domain::invoice::invoice_entity::Column::GroupId.eq(group_id))
            .filter(crate::domain::invoice::invoice_entity::Column::CreatedDate.between(now - chrono::Duration::days(7), now))
            .all(db)
            .await?;

            let mut invoices_weekly: Vec<InvoiceWeeklyDto> = Vec::new();

            for invoice in invoices {
                let meal = Meal::find_by_id(invoice.meal_id).one(db).await?;
                let meal_name = meal.name;
                let week_day = invoice.created_date.weekday().to_string();
                invoices_weekly.push(InvoiceWeeklyDto { id: invoice.id, week_day, meal_name, created_date: invoice.created_date });
            }

        Ok(invoices)
    }
} 