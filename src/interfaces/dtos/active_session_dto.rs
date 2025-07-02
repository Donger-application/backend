#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActiveSessionDto {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateActiveSessionDto {
    pub user_id: i32,
    pub group_id: i32,
} 