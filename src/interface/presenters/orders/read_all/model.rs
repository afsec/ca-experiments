use super::ReadAll;
use crate::{
    interface::{
        presenters::Model,
        repositories::order::{read_all::Order, OrderRepo},
    },
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, (), Vec<Order>> for ReadAll {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        _submitted_data: (),
    ) -> AppResult<Vec<Order>> {
        OrderRepo::read_all(db_conn_pool).await
    }
}
