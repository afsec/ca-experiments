use super::ReadAll;
use crate::{
    interface::{presenters::Model, repositories::author::{AuthorRepo, find_all::Author}},
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, (), Vec<Author>> for ReadAll {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        submitted_data: (),
    ) -> AppResult<Vec<Author>> {
        AuthorRepo::find_all(db_conn_pool).await
        // Ok(vec![])
    }
}
