#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MealDto {
    pub id: i32,
    pub product_id: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateMealDto {
    pub product_id: i32,
} 