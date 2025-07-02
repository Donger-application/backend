#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomerDto {
    pub id: i32,
    pub user_id: i32,
    pub balance: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateCustomerDto {
    pub user_id: i32,
    pub balance: i32,
} 