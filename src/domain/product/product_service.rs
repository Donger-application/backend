use crate::domain::product::product_entity::{Entity as Product, Model as ProductModel};
use sea_orm::{DatabaseConnection, EntityTrait};

pub struct ProductService;

impl ProductService {
    pub async fn exists_by_id(db: &DatabaseConnection, id: i32) -> Result<bool, sea_orm::DbErr> {
        Ok(Product::find_by_id(id).one(db).await?.is_some())
    }

    pub async fn get_all_products(
        db: &DatabaseConnection,
    ) -> Result<Vec<ProductModel>, sea_orm::DbErr> {
        Product::find().all(db).await
    }
}
