#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GroupDto {
    pub id: i32,
    pub name: String,
    pub is_public: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateGroupDto {
    pub name: String,
    pub is_public: bool,
} 