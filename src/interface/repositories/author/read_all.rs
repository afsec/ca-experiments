use super::AuthorRepo;
use crate::{
    domain::entities::author::{AuthorId, AuthorName},
    AppResult,
};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

// * DepartmentFromSqlx
#[derive(Debug, FromRow)]
pub(crate) struct AuthorFromSQLx {
    id: i64,
    name: String,
}

impl TryFrom<Author> for AuthorFromSQLx {
    type Error = anyhow::Error;

    fn try_from(author: Author) -> Result<Self, Self::Error> {
        Ok(Self {
            id: author.id.try_into()?,
            name: author.name.try_into()?,
        })
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct Author {
    id: AuthorId,
    name: AuthorName,
}

impl TryFrom<AuthorFromSQLx> for Author {
    type Error = anyhow::Error;

    fn try_from(author: AuthorFromSQLx) -> Result<Self, Self::Error> {
        Ok(Self {
            id: author.id.try_into()?,
            name: author.name.try_into()?,
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

        let authors: AppResult<Vec<Author>> = records
            .into_iter()
            .map(|record| Ok(record.try_into()?))
            .collect();
        tracing::debug!("Authors: {:?}", &authors);
        Ok(authors?)
    }
}
