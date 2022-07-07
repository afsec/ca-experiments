use super::OrderRepo;
use crate::{
    domain::entities::order::{OrderId, OrderName},
    AppResult,
};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

// * DepartmentFromSqlx
#[derive(Debug, FromRow)]
pub(crate) struct OrderFromSQLx {
    id: i64,
    name: String,
}

impl TryFrom<Order> for OrderFromSQLx {
    type Error = anyhow::Error;

    fn try_from(order: Order) -> Result<Self, Self::Error> {
        Ok(Self {
            id: order.id.try_into()?,
            name: order.name.try_into()?,
        })
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct Order {
    id: OrderId,
    name: OrderName,
}

impl TryFrom<OrderFromSQLx> for Order {
    type Error = anyhow::Error;

    fn try_from(order: OrderFromSQLx) -> Result<Self, Self::Error> {
        Ok(Self {
            id: order.id.try_into()?,
            name: order.name.try_into()?,
        })
    }
}

impl OrderRepo {
    pub(crate) async fn count(db_conn_pool: &SqlitePool) -> AppResult<OrderLength> {
        let records = sqlx::query!(
            r#"
                SELECT COUNT(*) as count FROM `orders`;
            "#
        )
        .fetch_one(db_conn_pool)
        .await?;
        Ok(OrderLength::try_from(records.count)?)
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct OrderLength(u32);

impl From<OrderLength> for String {
    fn from(data: OrderLength) -> Self {
        data.0.to_string()
    }
}

impl TryFrom<i32> for OrderLength {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}
