use super::CartRepo;
use crate::{
    domain::entities::cart::{CartId, CartName},
    AppResult,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct NewCart {
    pub(crate) name: CartName,
}

impl CartRepo {
    pub(crate) async fn create(db_conn_pool: &SqlitePool, new_cart: NewCart) -> AppResult<CartId> {
        let rowid = sqlx::query!(
            r#"
                INSERT INTO `carts` ("name")
                VALUES (?1);
            "#,
            *new_cart.name,
        )
        .execute(db_conn_pool)
        .await?
        .last_insert_rowid();
        Ok(rowid.try_into()?)
    }
}
