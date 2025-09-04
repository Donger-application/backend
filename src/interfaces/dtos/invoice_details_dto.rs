#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InvoiceDetailsDto {
    pub invoice_id: i32,
    pub product_id: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateInvoiceDetailsDto {
    pub invoice_id: i32,
    pub product_id: i32,
} 