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
    pub(crate) async fn count(db_conn_pool: &SqlitePool) -> AppResult<CartLength> {
        let records = sqlx::query!(
            r#"
                SELECT COUNT(*) as count FROM `carts`;
            "#
        )
        .fetch_one(db_conn_pool)
        .await?;
        Ok(CartLength::try_from(records.count)?)
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct CartLength(u32);

impl From<CartLength> for String {
    fn from(data: CartLength) -> Self {
        data.0.to_string()
    }
}

impl TryFrom<i32> for CartLength {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}
