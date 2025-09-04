use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub data: T,
    pub error_message: String,
}

impl<T> ApiResponse<T> {
    pub fn new(status: u16, data: T, error_message: impl Into<String>) -> Self {
        Self {
            status,
            data,
            error_message: error_message.into(),
        }
    }
} 