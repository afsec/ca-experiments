use super::ReadAll;
use crate::{
    interface::{
        presenters::Model,
        repositories::cart::{read_all::Cart, CartRepo},
    },
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, (), Vec<Cart>> for ReadAll {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        _submitted_data: (),
    ) -> AppResult<Vec<Cart>> {
        CartRepo::read_all(db_conn_pool).await
    }
}
