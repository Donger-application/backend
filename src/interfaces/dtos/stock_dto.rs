#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StockDto {
    pub id: i32,
    pub price: i64,
    pub consumed: bool,
    pub product_id: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateStockDto {
    pub price: i64,
    pub consumed: bool,
    pub product_id: i32,
} 