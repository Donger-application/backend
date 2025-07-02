#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProductDto {
    pub id: i32,
    pub name: String,
    pub group_id: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateProductDto {
    pub name: String,
    pub group_id: i32,
} 