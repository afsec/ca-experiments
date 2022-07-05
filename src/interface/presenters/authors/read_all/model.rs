use super::ReadAll;
use crate::{
    interface::{
        presenters::Model,
        repositories::author::{read_all::Author, AuthorRepo},
    },
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, (), Vec<Author>> for ReadAll {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        _submitted_data: (),
    ) -> AppResult<Vec<Author>> {
        AuthorRepo::read_all(db_conn_pool).await
    }
}
