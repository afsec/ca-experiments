use super::Create;
use crate::domain::entities::cart::CartId;
use crate::interface::repositories::cart::create::NewCart;
use crate::{
    interface::{presenters::Model, repositories::cart::CartRepo},
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, NewCart, CartId> for Create {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        submitted_data: NewCart,
    ) -> AppResult<CartId> {
        CartRepo::create(db_conn_pool, submitted_data).await
    }
}
