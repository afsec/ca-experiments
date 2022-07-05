use super::Create;
use crate::domain::entities::author::AuthorId;

use crate::usecases::author::NewAuthor;
use crate::{
    interface::{presenters::Model, repositories::author::AuthorRepo},
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, NewAuthor, AuthorId> for Create {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        submitted_data: NewAuthor,
    ) -> AppResult<AuthorId> {
        AuthorRepo::create(db_conn_pool, submitted_data).await
    }
}
