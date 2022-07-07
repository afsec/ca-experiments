use super::Count;
use crate::{
    interface::{
        presenters::Model,
        repositories::cart::{count::CartLength, CartRepo},
    },
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, (), CartLength> for Count {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        _submitted_data: (),
    ) -> AppResult<CartLength> {
        CartRepo::count(db_conn_pool).await
    }
}
