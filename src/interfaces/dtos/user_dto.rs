use validator::Validate;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserDto {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
    pub email_confirmed: bool,
    pub user_display_id: String,
    pub balance: i32,
    pub is_active: bool,
    pub role_id: i32,
    pub group_id: i32,
    pub created_date: chrono::NaiveDate,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Validate)]
pub struct CreateUserDto {
    pub name: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub email_confirmed: bool,
    #[validate(length(min = 3, message = "User display ID must be at least 3 characters long"))]
    pub user_display_id: String,
    pub balance: i32,
    pub is_active: bool,
    pub role_id: i32,
    pub group_id: i32,
    pub created_date: chrono::NaiveDate,
}