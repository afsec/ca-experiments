use super::Count;
use crate::{
    interface::{
        presenters::Model,
        repositories::book::{count::BookLength, BookRepo},
    },
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, (), BookLength> for Count {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        _submitted_data: (),
    ) -> AppResult<BookLength> {
        BookRepo::count(db_conn_pool).await
    }
}
