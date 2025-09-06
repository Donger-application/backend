use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]

pub struct SupplierDto {
    pub id: i32,
    pub name: String,
}