use actix_web::Result;
use sqlx::{Pool, Postgres};
use sqlx_clean_querybuilder::{qb::select::Order, query_builder::PostgreSqlQueryBuilder};

use crate::interfaces::dtos::user_dto::UserDisplayDto;

pub struct UserService;

impl UserService {
    pub async fn get_user_indebt(group_id: i32, db: Pool<Postgres>) -> Result<Vec<UserDisplayDto>> {
        let condition = format!("group_id = {} AND is_active = true", group_id);


        let qq = PostgreSqlQueryBuilder::select()
            .columns(&["id", "name", "balance"])
            .filter(&condition)
            .table("user", None)
            .order_by("balance", Order::Asc)
            .build();

        let users: Vec<UserDisplayDto> = sqlx::query_as::<_, UserDisplayDto>(&qq)
            .fetch_all(&db)
            .await
            .unwrap();

        Ok(users)
    }
}
