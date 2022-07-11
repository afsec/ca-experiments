use super::CartRepo;
use crate::{
    domain::entities::{book::BookId, cart::CartId, order::OrderId},
    AppResult,
};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

// * DepartmentFromSqlx
#[derive(Debug, FromRow)]
pub(crate) struct CartItemFromSQLx {
    id: i64,
    order_id: i64,
    cart_id: i64,
    book_id: i64,
}

impl TryFrom<CartItem> for CartItemFromSQLx {
    type Error = anyhow::Error;

    fn try_from(item: CartItem) -> Result<Self, Self::Error> {
        Ok(Self {
            id: item.id.try_into()?,
            order_id: item.order.try_into()?,
            cart_id: item.cart.try_into()?,
            book_id: item.book.try_into()?,
        })
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct CartItem {
    id: CartId,
    order: OrderId,
    cart: CartId,
    book: BookId,
}

impl TryFrom<CartItemFromSQLx> for CartItem {
    type Error = anyhow::Error;

    fn try_from(item: CartItemFromSQLx) -> Result<Self, Self::Error> {
        Ok(Self {
            id: item.id.try_into()?,
            order: item.order_id.try_into()?,
            cart: item.cart_id.try_into()?,
            book: item.book_id.try_into()?,
        })
    }
}

impl CartRepo {
    pub(crate) async fn read_all_items(
        db_conn_pool: &SqlitePool,
        cart_id: &CartId,
    ) -> AppResult<Vec<CartItem>> {
        let records: Vec<CartItemFromSQLx> = sqlx::query_as!(
            CartItemFromSQLx,
            r#"
            SELECT id,order_id,cart_id,book_id
            FROM `cart_items` WHERE cart_id = ?1;
        "#,
            **cart_id
        )
        .fetch_all(db_conn_pool)
        .await?;

        // * To improve performance -> https://github.com/launchbadge/sqlx/issues/117

        let carts: AppResult<Vec<CartItem>> = records
            .into_iter()
            .map(|record| Ok(record.try_into()?))
            .collect();
        tracing::debug!("Carts: {:?}", &carts);
        Ok(carts?)
    }
}
