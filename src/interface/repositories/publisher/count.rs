use crate::{
    domain::entities::publisher::structs::Publisher, interface::repositories::Repository, AppResult,
};
use async_trait::async_trait;
use serde::Serialize;
use sqlx::{FromRow, Sqlite};

// * DepartmentFromSqlx
#[derive(Debug, FromRow)]
pub(crate) struct PublisherFromSQLx {
    id: i64,
    name: String,
}

impl TryFrom<Publisher> for PublisherFromSQLx {
    type Error = anyhow::Error;

    fn try_from(publisher: Publisher) -> Result<Self, Self::Error> {
        let (id, name) = publisher.into();
        Ok(Self {
            id: id.try_into()?,
            name: name.try_into()?,
        })
    }
}

impl TryFrom<PublisherFromSQLx> for Publisher {
    type Error = anyhow::Error;

    fn try_from(publisher: PublisherFromSQLx) -> Result<Self, Self::Error> {
        Ok(Self::from((
            publisher.id.try_into()?,
            publisher.name.into(),
        )))
    }
}

pub(crate) struct RepoPublisherCount;
#[async_trait]
impl<'endpoint> Repository<Sqlite, (), PublisherLength> for RepoPublisherCount {
    async fn repository(
        db_conn_pool: &sqlx::Pool<Sqlite>,
        _submitted_data: (),
    ) -> AppResult<PublisherLength> {
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
