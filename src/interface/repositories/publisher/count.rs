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
    pub(crate) async fn count(db_conn_pool: &SqlitePool) -> AppResult<PublisherLength> {
        let records = sqlx::query!(
            r#"
                SELECT COUNT(*) as count FROM `publishers`;
            "#
        )
        .fetch_one(db_conn_pool)
        .await?;
        Ok(PublisherLength::try_from(records.count)?)
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct PublisherLength(u32);

impl From<PublisherLength> for String {
    fn from(data: PublisherLength) -> Self {
        data.0.to_string()
    }
}

impl TryFrom<i32> for PublisherLength {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}
