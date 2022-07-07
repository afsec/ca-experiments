use super::Count;
use crate::{
    interface::{
        presenters::Model,
        repositories::order::{count::OrderLength, OrderRepo},
    },
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, (), OrderLength> for Count {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        _submitted_data: (),
    ) -> AppResult<OrderLength> {
        OrderRepo::count(db_conn_pool).await
    }
}
