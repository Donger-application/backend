#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OrderCustomerDto {
    pub order_id: i32,
    pub customer_id: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateOrderCustomerDto {
    pub order_id: i32,
    pub customer_id: i32,
} 