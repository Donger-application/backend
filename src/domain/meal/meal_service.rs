use crate::domain::meal::meal_entity::{ActiveModel as MealActiveModel, Entity as Meal, Model as MealModel};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct MealService;

impl MealService {
    pub async fn get_all_meals(db: &DatabaseConnection) -> Result<Vec<MealModel>, sea_orm::DbErr> {
        Meal::find().all(db).await
    }

    pub async fn get_meal_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<MealModel>, sea_orm::DbErr> {
        Meal::find_by_id(id).one(db).await
    }

    pub async fn get_meals_by_product_id(db: &DatabaseConnection, product_id: i32) -> Result<Vec<MealModel>, sea_orm::DbErr> {
        Meal::find().filter(crate::domain::meal::meal_entity::Column::ProductId.eq(product_id)).all(db).await
    }

    pub async fn create_meal(db: &DatabaseConnection, product_id: i32) -> Result<MealModel, sea_orm::DbErr> {
        // Check if meal with this product_id already exists (optional, remove if not needed)
        // let found_meal = Meal::find().filter(crate::domain::meal::meal_entity::Column::ProductId.eq(product_id)).one(db).await?;
        // if found_meal.is_some() {
        //     return Err(sea_orm::DbErr::Custom("meal with this product_id already exists".to_string()));
        // }
        let active_model = MealActiveModel {
            product_id: Set(product_id),
            ..Default::default()
        };
        active_model.insert(db).await
    }

    pub async fn update_meal(db: &DatabaseConnection, id: i32, product_id: Option<i32>) -> Result<Option<MealModel>, sea_orm::DbErr> {
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
} 