#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SupplierDto {
    pub id: i32,
    pub balance: i32,
    pub user_id: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateSupplierDto {
    pub balance: i32,
    pub user_id: i32,
} 