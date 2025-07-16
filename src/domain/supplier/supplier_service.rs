use crate::domain::supplier::supplier_entity::{Entity as Supplier};
use sea_orm::{DatabaseConnection, EntityTrait};

pub struct SupplierService;

impl SupplierService {
    pub async fn exists_by_id(db: &DatabaseConnection, id: i32) -> Result<bool, sea_orm::DbErr> {
        Ok(Supplier::find_by_id(id).one(db).await?.is_some())
    }
} 