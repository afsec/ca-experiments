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
    pub(crate) async fn read_all(db_conn_pool: &SqlitePool) -> AppResult<Vec<Order>> {
        let records: Vec<OrderFromSQLx> = sqlx::query_as!(
            OrderFromSQLx,
            r#"
            SELECT id, name
            FROM `orders`;
        "#,
        )
        .fetch_all(db_conn_pool)
        .await?;

        // * To improve performance -> https://github.com/launchbadge/sqlx/issues/117

        let orders: AppResult<Vec<Order>> = records
            .into_iter()
            .map(|record| Ok(record.try_into()?))
            .collect();
        tracing::debug!("Orders: {:?}", &orders);
        Ok(orders?)
    }
}
