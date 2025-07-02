#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InvoiceDetailsDto {
    pub id: i32,
    pub invoice_id: i32,
    pub stock_id: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateInvoiceDetailsDto {
    pub invoice_id: i32,
    pub stock_id: i32,
} 