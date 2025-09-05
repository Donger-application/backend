use sqlx::PgPool;
use sqlx_clean_querybuilder::{qb::select::Order, query_builder::PostgreSqlQueryBuilder};

use crate::interfaces::dtos::invoice_dto::InvoiceDto;

pub struct InvoiceService;

impl InvoiceService {
    // pub async fn get_invoice_report(group_id: i32, db:Pool<PgPool>) {
    //     //-- select invoice.id, price, created_date,u.id, u.name, string_agg(DISTINCT p.name, ', ' ORDER BY p.name)
    //     //-- from "invoice"
    //     // inner join "invoice_details" ind on invoice.id = ind.invoice_id
    //     // inner join "meal" on meal.id = invoice.meal_id
    //     // inner join "meal_product" mp on meal.id = mp.meal_id
    //     // inner join "product" p on p.id = mp.product_id
    //     // inner join "supplier" s on s.id = supplier_id
    //     // inner join "user" u on u.id = s.user_id
    //     //-- where is_deleted = false and invoice.group_id = 1
    //     // group by invoice.id, price, created_date, u.name, meal.id, u.id;

    //     let condition = format!("invoice.group_id = {} AND is_deleted = false", group_id);


    //     let qq = PostgreSqlQueryBuilder::select()
    //         .columns(&["invoice.id", "price", "created_date","u.id", "u.name", "string_agg(DISTINCT p.name, ', ' ORDER BY p.name)"])
    //         .filter(&condition)
    //         .table("invoice", None)
    //         .order_by("balance", Order::Asc)
    //         .build();

    //     let users: Vec<InvoiceDto> = sqlx::query_as::<_, InvoiceDto>(&qq)
    //         .fetch_all(&db)
    //         .await
    //         .unwrap();

    //     Ok(users)
    // }
}
