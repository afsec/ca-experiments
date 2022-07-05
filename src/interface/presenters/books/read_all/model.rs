use super::ReadAll;
use crate::{
    interface::{
        presenters::Model,
        repositories::book::{read_all::Book, BookRepo},
    },
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, (), Vec<Book>> for ReadAll {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        submitted_data: (),
    ) -> AppResult<Vec<Book>> {
        BookRepo::read_all(db_conn_pool).await
    }
}
