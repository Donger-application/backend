use crate::domain::invoice_details::invoice_details_entity::{ActiveModel as InvoiceDetailsActiveModel,  Model as InvoiceDetailsModel};
use crate::domain::stock::stock_entity::{ActiveModel as StockActiveModel};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

pub struct InvoiceDetailsService;

impl InvoiceDetailsService {
    pub async fn create_invoice_details_with_products(
        db: &DatabaseConnection,
        invoice_id: i32,
        product_ids: Vec<i32>,
    ) -> Result<Vec<InvoiceDetailsModel>, sea_orm::DbErr> {
        let mut created_details = Vec::new();
        for product_id in product_ids {
            // Create stock with default values
            let stock_active = StockActiveModel {
                price: Set(0),
                consumed: Set(false),
                product_id: Set(product_id),
                ..Default::default()
            };
            let stock = stock_active.insert(db).await?;
            // Create invoice detail with the generated stock_id
            let detail_active = InvoiceDetailsActiveModel {
                invoice_id: Set(invoice_id),
                stock_id: Set(stock.id),
                ..Default::default()
            };
            let detail = detail_active.insert(db).await?;
            created_details.push(detail);
        }
        Ok(created_details)
    }
} 