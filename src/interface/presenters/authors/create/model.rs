use super::Create;
use crate::{
    interface::{presenters::Model, repositories::author::{AuthorRepo, find_all::Author}},
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, String, Vec<Author>> for Create {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        submitted_data: String,
    ) -> AppResult<Vec<Author>> {
        AuthorRepo::create(db_conn_pool,).await
        // Ok(vec![])
    }
}
