use super::OrderRepo;
use crate::{
    domain::entities::order::{OrderId, OrderName},
    AppResult,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct NewOrder {
    pub(crate) name: OrderName,
}

impl OrderRepo {
    pub(crate) async fn create(
        db_conn_pool: &SqlitePool,
        new_order: NewOrder,
    ) -> AppResult<OrderId> {
        let rowid = sqlx::query!(
            r#"
                INSERT INTO `orders` ("name")
                VALUES (?1);
            "#,
            *new_order.name,
        )
        .execute(db_conn_pool)
        .await?
        .last_insert_rowid();
        Ok(rowid.try_into()?)
    }
}
