use crate::domain::invoice::invoice_entity;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InvoiceDto {
    pub id: i32,
    pub price: i64,
    pub is_deleted: bool,
    pub deleted_by: i32,
    pub created_date: chrono::NaiveDateTime,
    pub last_modification_date: chrono::NaiveDateTime,
    pub meal_id: i32,
    pub group_id: i32,
    pub supplier_id: i32,
}

impl From<invoice_entity::Model> for InvoiceDto {
    fn from(model: invoice_entity::Model) -> Self {
        InvoiceDto {
            id: model.id,
            price: model.price,
            is_deleted: model.is_deleted,
            deleted_by: model.deleted_by,
            created_date: model.created_date,
            last_modification_date: model.last_modification_date,
            meal_id: model.meal_id,
            group_id: model.group_id,
            supplier_id: model.supplier_id,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateInvoiceDto {
    pub price: i64,
    pub is_deleted: bool,
    pub deleted_by: i32,
    pub created_date: chrono::NaiveDateTime,
    pub last_modification_date: chrono::NaiveDateTime,
    pub meal_id: i32,
    pub group_id: i32,
    pub supplier_id: i32,
} 