use super::PublisherRepo;
use crate::{
    domain::entities::publisher::{PublisherId, PublisherName},
    AppResult,
};
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct NewPublisher {
    pub(crate) name: PublisherName,
}

impl PublisherRepo {
    pub(crate) async fn create(
        db_conn_pool: &SqlitePool,
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
