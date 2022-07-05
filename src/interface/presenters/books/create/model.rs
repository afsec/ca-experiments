use super::Create;
use crate::domain::entities::book::BookId;
use crate::interface::repositories::book::create::NewBook;
use crate::{
    interface::{presenters::Model, repositories::book::BookRepo},
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, NewBook, BookId> for Create {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        submitted_data: NewBook,
    ) -> AppResult<BookId> {
        BookRepo::create(db_conn_pool, submitted_data).await
    }
}
