use actix_web::Result;
use sqlx::{Pool, Postgres};
use sqlx_clean_querybuilder::query_builder::PostgreSqlQueryBuilder;

use crate::interfaces::dtos::invoice_dto::{group_invoices, InvoiceResponse, InvoiceRow};

pub struct InvoiceService;

impl InvoiceService {
    pub async fn get_invoice_report(
        group_id: i32,
        start_date: String,
        end_date: String,
        db: Pool<Postgres>,
    ) -> Result<Vec<InvoiceResponse>> {
        let condition = format!("invoice.group_id = {} AND is_deleted = false AND created_date::date BETWEEN '{}' and '{}'", group_id, start_date, end_date);

        let qq = PostgreSqlQueryBuilder::select()
            .columns(&[
                "invoice.id",
                "price",
                "created_date",
                "u.id as supplier_id",
                "u.name as supplier_name",
                "string_agg(DISTINCT p.name, ', ' ORDER BY p.name) as meal",
                "supplier_id",
            ])
            .table("invoice", None)
            .join_inner(
                "invoice_details",
                Some("ind"),
                "invoice.id",
                "ind.invoice_id",
            )
            .join_inner("meal", None, "meal.id", "invoice.meal_id")
            .join_inner("meal_product", Some("mp"), "meal.id", "mp.meal_id")
            .join_inner("product", Some("p"), "p.id", "mp.product_id")
            .join_inner("supplier", Some("s"), "s.id", "supplier_id")
            .join_inner("\"user\"", Some("u"), "u.id", "s.user_id")
            .filter(&condition)
            .group_by("invoice.id, price, created_date, u.name, meal.id, u.id, supplier_id")
            .build();

        println!("{}", qq);

        // fetch from database
        let invoices: Vec<InvoiceRow> = sqlx::query_as::<_, InvoiceRow>(&qq)
            .fetch_all(&db)
            .await
            .unwrap();

        let response = group_invoices(invoices);

        Ok(response)
    }
}
