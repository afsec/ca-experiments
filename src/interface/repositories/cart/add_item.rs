use super::CartRepo;
use crate::{
    domain::entities::{book::BookId, cart::CartId, order::OrderId},
    AppResult,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct NewItem {
    pub(crate) order: OrderId,
    pub(crate) cart: CartId,
    pub(crate) book: BookId,
}

impl CartRepo {
    pub(crate) async fn add_item(
        db_conn_pool: &SqlitePool,
        new_cart: NewItem,
    ) -> AppResult<CartId> {
        let rowid = sqlx::query!(
            r#"
                INSERT INTO `cart_items` ("order_id","cart_id","book_id")
                VALUES (?1,?2,?3);
            "#,
            *new_cart.order,
            *new_cart.cart,
            *new_cart.book,
        )
        .execute(db_conn_pool)
        .await?
        .last_insert_rowid();
        Ok(rowid.try_into()?)
    }
}
