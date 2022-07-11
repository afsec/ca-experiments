use crate::{
    domain::entities::publisher::structs::Publisher, interface::repositories::Repository, AppResult,
};
use async_trait::async_trait;
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
        Ok(Self {
            id: publisher.id.try_into()?,
            name: publisher.name.try_into()?,
        })
    }
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

pub(crate) struct RepoPublisherReadAll;
#[async_trait]
impl<'endpoint> Repository<Sqlite, (), Vec<Publisher>> for RepoPublisherReadAll {
    async fn repository(
        db_conn_pool: &sqlx::Pool<Sqlite>,
        _params: (),
    ) -> AppResult<Vec<Publisher>> {
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
