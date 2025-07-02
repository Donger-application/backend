#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OrderDetailsDto {
    pub id: i32,
    pub order_id: i32,
    pub stock_id: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateOrderDetailsDto {
    pub order_id: i32,
    pub stock_id: i32,
} 