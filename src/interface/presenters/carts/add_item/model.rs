use super::AddItem;
use crate::domain::entities::cart::CartId;
use crate::interface::repositories::cart::add_item::NewItem;
use crate::{
    interface::{presenters::Model, repositories::cart::CartRepo},
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, NewItem, CartId> for AddItem {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        submitted_data: NewItem,
    ) -> AppResult<CartId> {
        CartRepo::add_item(db_conn_pool, submitted_data).await
    }
}
