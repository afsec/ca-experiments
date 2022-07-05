use super::Count;
use crate::{
    interface::{
        presenters::Model,
        repositories::publisher::{count::PublisherLength, PublisherRepo},
    },
    AppResult,
};
use async_trait::async_trait;
use sqlx::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, (), PublisherLength> for Count {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        _submitted_data: (),
    ) -> AppResult<PublisherLength> {
        PublisherRepo::count(db_conn_pool).await
    }
}
