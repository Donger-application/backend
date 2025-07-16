use crate::domain::meal::meal_entity::{
    ActiveModel as MealActiveModel, Entity as Meal, Model as MealModel,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct MealService;

impl MealService {
    pub async fn get_all_meals(db: &DatabaseConnection) -> Result<Vec<MealModel>, sea_orm::DbErr> {
        Meal::find().all(db).await
    }

    pub async fn get_meal_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<MealModel>, sea_orm::DbErr> {
        Meal::find_by_id(id).one(db).await
    }

    pub async fn get_meals_by_product_id(
        db: &DatabaseConnection,
        product_id: i32,
    ) -> Result<Vec<MealModel>, sea_orm::DbErr> {
        Meal::find()
            .filter(crate::domain::meal::meal_entity::Column::ProductId.eq(product_id))
            .all(db)
            .await
    }

    pub async fn create_meal(
        db: &DatabaseConnection,
        product_id: i32,
    ) -> Result<MealModel, sea_orm::DbErr> {
        let active_model = MealActiveModel {
            product_id: Set(product_id),
            ..Default::default()
        };
        active_model.insert(db).await
    }

    pub async fn create_meal_with_products(
        db: &DatabaseConnection,
        product_ids: &[i32],
    ) -> Result<Vec<MealModel>, sea_orm::DbErr> {
        let mut created_meals = Vec::new();
        for product_id in product_ids {
            let meal = MealActiveModel {
                product_id: Set(*product_id),
                ..Default::default()
            };
            let inserted = meal.insert(db).await?;
            created_meals.push(inserted);
        }
        Ok(created_meals)
    }

    pub async fn update_meal(
        db: &DatabaseConnection,
        id: i32,
        product_id: Option<i32>,
    ) -> Result<Option<MealModel>, sea_orm::DbErr> {
        if let Some(meal) = Meal::find_by_id(id).one(db).await? {
            let mut active_model: MealActiveModel = meal.into();
            if let Some(product_id) = product_id {
                active_model.product_id = Set(product_id);
            }
            let updated = active_model.update(db).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_meal(db: &DatabaseConnection, id: i32) -> Result<bool, sea_orm::DbErr> {
        let result = Meal::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn exists_by_id(db: &DatabaseConnection, id: i32) -> Result<bool, sea_orm::DbErr> {
        Ok(Meal::find_by_id(id).one(db).await?.is_some())
    }

    pub async fn exists_with_exact_product_ids(
        db: &DatabaseConnection,
        product_ids: &[i32],
    ) -> Result<bool, sea_orm::DbErr> {
        // Find all meals that have all the given product_ids
        let meals = Meal::find()
            .filter(crate::domain::meal::meal_entity::Column::ProductId.is_in(product_ids.to_vec()))
            .all(db)
            .await?;
        // Group meals by meal id
        use std::collections::HashMap;
        let mut meal_to_products: HashMap<i32, Vec<i32>> = HashMap::new();
        for meal in meals {
            meal_to_products
                .entry(meal.id)
                .or_default()
                .push(meal.product_id);
        }
        // Check if any meal has exactly the same set of product_ids
        for products in meal_to_products.values() {
            let mut sorted_db = products.clone();
            let mut sorted_input = product_ids.to_vec();
            sorted_db.sort();
            sorted_input.sort();
            if sorted_db == sorted_input {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
