use super::CartRepo;
use crate::{
    domain::entities::cart::{CartId, CartName},
    AppResult,
};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

// * DepartmentFromSqlx
#[derive(Debug, FromRow)]
pub(crate) struct CartFromSQLx {
    id: i64,
    name: String,
}

impl TryFrom<Cart> for CartFromSQLx {
    type Error = anyhow::Error;

    fn try_from(cart: Cart) -> Result<Self, Self::Error> {
        Ok(Self {
            id: cart.id.try_into()?,
            name: cart.name.try_into()?,
        })
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct Cart {
    id: CartId,
    name: CartName,
}

impl TryFrom<CartFromSQLx> for Cart {
    type Error = anyhow::Error;

    fn try_from(cart: CartFromSQLx) -> Result<Self, Self::Error> {
        Ok(Self {
            id: cart.id.try_into()?,
            name: cart.name.try_into()?,
        })
    }
}

impl CartRepo {
    pub(crate) async fn read_all(db_conn_pool: &SqlitePool) -> AppResult<Vec<Cart>> {
        let records: Vec<CartFromSQLx> = sqlx::query_as!(
            CartFromSQLx,
            r#"
            SELECT id, name
            FROM `carts`;
        "#,
        )
        .fetch_all(db_conn_pool)
        .await?;

        // * To improve performance -> https://github.com/launchbadge/sqlx/issues/117

        let carts: AppResult<Vec<Cart>> = records
            .into_iter()
            .map(|record| Ok(record.try_into()?))
            .collect();
        tracing::debug!("Carts: {:?}", &carts);
        Ok(carts?)
    }
}
