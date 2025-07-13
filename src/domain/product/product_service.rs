use crate::domain::product::product_entity::{Entity as ProductEntity, Model as ProductModel};
use sea_orm::{DatabaseConnection, EntityTrait};

pub struct ProductService;

impl ProductService {
    pub async fn get_all_products(db: &DatabaseConnection) -> Result<Vec<ProductModel>, sea_orm::DbErr> {
        ProductEntity::find().all(db).await
    }
}
