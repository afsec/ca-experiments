use crate::{
    domain::entities::publisher::{
        fields::{PublisherId, PublisherName},
        structs::PublisherToBeValidated,
    },
    interface::{repositories::Repository, services::FieldSealed},
    AppResult,
};
use async_trait::async_trait;
use sqlx::{FromRow, Sqlite};

#[derive(Debug, FromRow)]
pub(crate) struct PublisherFromSQLx {
    id: i64,
    name: String,
}
impl TryFrom<PublisherFromSQLx> for PublisherToBeValidated {
    type Error = anyhow::Error;

    fn try_from(value: PublisherFromSQLx) -> Result<Self, Self::Error> {
        let PublisherFromSQLx { id, name } = value;
        Ok(Self::from((
            FieldSealed::from(PublisherId::try_from(id)?),
            FieldSealed::from(PublisherName::from(name)),
        )))
    }
}

// impl TryFrom<Publisher> for PublisherFromSQLx {
//     type Error = anyhow::Error;

//     fn try_from(publisher: Publisher) -> Result<Self, Self::Error> {
//         Ok(Self {
//             id: publisher.id.try_into()?,
//             name: publisher.name.try_into()?,
//         })
//     }
// }

// impl TryFrom<PublisherFromSQLx> for Publisher {
//     type Error = anyhow::Error;

//     fn try_from(publisher: PublisherFromSQLx) -> Result<Self, Self::Error> {
//         Ok(Self {
//             id: publisher.id.try_into()?,
//             name: publisher.name.try_into()?,
//         })
//     }
// }

pub(crate) struct RepoPublisherReadAll;
#[async_trait]
impl<'endpoint> Repository<Sqlite, (), Vec<PublisherToBeValidated>> for RepoPublisherReadAll {
    async fn repository(
        db_conn_pool: &sqlx::Pool<Sqlite>,
        _params: (),
    ) -> AppResult<Vec<PublisherToBeValidated>> {
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

        let publishers: AppResult<Vec<PublisherToBeValidated>> = records
            .into_iter()
            .map(|record| Ok(record.try_into()?))
            .collect();
        tracing::debug!("Publishers: {:?}", &publishers);
        Ok(publishers?)
    }
}
