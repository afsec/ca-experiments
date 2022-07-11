use crate::{
    domain::entities::publisher::fields::{PublisherId, PublisherName},
    interface::repositories::Repository,
    AppResult,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::Sqlite;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct NewPublisher {
    pub(crate) name: PublisherName,
}

pub(crate) struct RepoPublisherCreate;
#[async_trait]
impl<'endpoint> Repository<Sqlite, NewPublisher, PublisherId> for RepoPublisherCreate {
    async fn repository(
        db_conn_pool: &sqlx::Pool<Sqlite>,
        new_publisher: NewPublisher,
    ) -> AppResult<PublisherId> {
        let rowid = sqlx::query!(
            r#"
                INSERT INTO `publishers` ("name")
                VALUES (?1);
            "#,
            *new_publisher.name,
        )
        .execute(db_conn_pool)
        .await?
        .last_insert_rowid();
        Ok(rowid.try_into()?)
    }
}
