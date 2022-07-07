use super::Create;
use crate::domain::entities::order::OrderId;
use crate::interface::repositories::order::create::NewOrder;
use crate::{
    interface::{presenters::Model, repositories::order::OrderRepo},
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, NewOrder, OrderId> for Create {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        submitted_data: NewOrder,
    ) -> AppResult<OrderId> {
        OrderRepo::create(db_conn_pool, submitted_data).await
    }
}
