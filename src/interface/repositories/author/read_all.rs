use super::AuthorRepo;
use crate::{
    domain::entities::author::{AuthorId, AuthorName},
    AppResult,
};

use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Serialize)]
pub(crate) struct Author {
    id: AuthorId,
    name: AuthorName,
}

#[derive(Debug, FromRow)]
struct AuthorFromSQLx {
    id: i64,
    name: String,
}
impl TryFrom<AuthorFromSQLx> for Author {
    type Error = anyhow::Error;

    fn try_from(value: AuthorFromSQLx) -> Result<Self, Self::Error> {
        let AuthorFromSQLx { id, name } = value;
        Ok(Self {
            id: AuthorId::try_from(id)?,
            name: AuthorName::from(name),
        })
    }
}
impl AuthorRepo {
    pub(crate) async fn read_all(db_conn_pool: &SqlitePool) -> AppResult<Vec<Author>> {
        let records: Vec<AuthorFromSQLx> = sqlx::query_as!(
            AuthorFromSQLx,
            r#"
            SELECT id, name
            FROM `authors`;
        "#,
        )
        .fetch_all(db_conn_pool)
        .await?;

        // * To improve performance -> https://github.com/launchbadge/sqlx/issues/117

        let authors: Vec<Author> = records
            .into_iter()
            .map(|record| {
                // Author::try_from(record).map_err(|error| {
                Author::try_from(record).map_err(|error| {
                    tracing::error!("{error:?} - (file: {}, line: {})", file!(), line!());
                    error
                })
            })
            .filter_map(|record: AppResult<Author>| record.ok())
            .collect();
        Ok(authors)
    }
}
