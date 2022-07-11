use super::ReadAll;
// use crate::interface::services::Service;
use crate::{
    domain::entities::publisher::structs::Publisher,
    interface::{
        presenters::Model, repositories::publisher::read_all::RepoPublisherReadAll,
        // services::publisher::ServicePublisherReadAll,
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
        submitted_data: (),
    ) -> AppResult<Vec<Publisher>> {
        use crate::interface::repositories::Repository;
        // ServicePublisherReadAll::service(db_conn_pool, RepoPublisherReadAll, submitted_data).await
        RepoPublisherReadAll::repository(db_conn_pool, submitted_data).await
    }
}
