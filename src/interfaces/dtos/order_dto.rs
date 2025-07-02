#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OrderDto {
    pub id: i32,
    pub is_deleted: bool,
    pub deleted_by: i32,
    pub created_date: chrono::NaiveDateTime,
    pub last_modification_date: chrono::NaiveDateTime,
    pub group_id: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateOrderDto {
    pub is_deleted: bool,
    pub deleted_by: i32,
    pub created_date: chrono::NaiveDateTime,
    pub last_modification_date: chrono::NaiveDateTime,
    pub group_id: i32,
} 