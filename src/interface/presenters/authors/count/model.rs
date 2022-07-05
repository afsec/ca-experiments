use super::{AuthorsFound, Count};
use crate::{
    interface::{presenters::Model, repositories::author::{AuthorRepo, count::AuthorLength}},
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, (), AuthorLength> for Count {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        submitted_data: (),
    ) -> AppResult<AuthorLength> {
        AuthorRepo::count(db_conn_pool).await
    }
}
