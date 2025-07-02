#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemLogDto {
    pub id: i32,
    pub transaction_type: String,
    pub description: String,
    pub date: chrono::NaiveDateTime,
    pub user_id: i32,
    pub group_id: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateSystemLogDto {
    pub transaction_type: String,
    pub description: String,
    pub date: chrono::NaiveDateTime,
    pub user_id: i32,
    pub group_id: i32,
} 