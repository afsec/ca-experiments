use super::ReadAll;

use crate::{
    domain::entities::publisher::structs::Publisher,
    interface::{presenters::Model, services::publisher::ServicePublisherReadAll},
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
        use crate::interface::services::Service;
        ServicePublisherReadAll::service(db_conn_pool, submitted_data).await
    }
}
