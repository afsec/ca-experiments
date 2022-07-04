use super::AuthorRepo;
use crate::{
    domain::entities::author::{AuthorId, AuthorName},
    AppResult,
};
use serde::Serialize;
use sqlx::{Database, FromRow, SqlitePool};

// * DepartmentFromSqlx
#[derive(Debug, FromRow)]
pub(crate) struct AuthorFromSQLx {
    id: i64,
    name: String,
}
impl From<Author> for AuthorFromSQLx {
    fn from(author: Author) -> Self {
        Self {
            id: author.id.into(),
            name: author.name.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct Author {
    id: AuthorId,
    name: AuthorName,
}

impl From<AuthorFromSQLx> for Author {
    fn from(author: AuthorFromSQLx) -> Self {
        Self {
            id: author.id.into(),
            name: author.name.into(),
        }
    }
}

// ? FromRow
// TODO:

impl AuthorRepo {
    pub(crate) async fn find_all(db_conn_pool: &SqlitePool) -> AppResult<Vec<Author>> {
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

        let authors: Vec<Author> = records.into_iter().map(|record| record.into()).collect();
        tracing::debug!("Authors: {:?}", &authors);
        Ok(authors)
    }
}
