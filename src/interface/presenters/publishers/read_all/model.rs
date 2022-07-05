use super::ReadAll;
use crate::{
    interface::{
        presenters::Model,
        repositories::publisher::{read_all::Publisher, PublisherRepo},
    },
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, (), Vec<Publisher>> for ReadAll {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        _submitted_data: (),
    ) -> AppResult<Vec<Publisher>> {
        PublisherRepo::read_all(db_conn_pool).await
    }
}
