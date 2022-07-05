use super::PublisherRepo;
use crate::{
    domain::entities::publisher::{PublisherId, PublisherName},
    AppResult,
};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

// * DepartmentFromSqlx
#[derive(Debug, FromRow)]
pub(crate) struct PublisherFromSQLx {
    id: i64,
    name: String,
}

impl TryFrom<Publisher> for PublisherFromSQLx {
    type Error = anyhow::Error;

    fn try_from(publisher: Publisher) -> Result<Self, Self::Error> {
        Ok(Self {
            id: publisher.id.try_into()?,
            name: publisher.name.try_into()?,
        })
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct Publisher {
    id: PublisherId,
    name: PublisherName,
}

impl TryFrom<PublisherFromSQLx> for Publisher {
    type Error = anyhow::Error;

    fn try_from(publisher: PublisherFromSQLx) -> Result<Self, Self::Error> {
        Ok(Self {
            id: publisher.id.try_into()?,
            name: publisher.name.try_into()?,
        })
    }
}

impl PublisherRepo {
    pub(crate) async fn read_all(db_conn_pool: &SqlitePool) -> AppResult<Vec<Publisher>> {
        let records: Vec<PublisherFromSQLx> = sqlx::query_as!(
            PublisherFromSQLx,
            r#"
            SELECT id, name
            FROM `publishers`;
        "#,
        )
        .fetch_all(db_conn_pool)
        .await?;

        // * To improve performance -> https://github.com/launchbadge/sqlx/issues/117

        let publishers: AppResult<Vec<Publisher>> = records
            .into_iter()
            .map(|record| Ok(record.try_into()?))
            .collect();
        tracing::debug!("Publishers: {:?}", &publishers);
        Ok(publishers?)
    }
}
