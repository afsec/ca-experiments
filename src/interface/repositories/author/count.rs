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
    pub(crate) async fn count(db_conn_pool: &SqlitePool) -> AppResult<AuthorLength> {
        let records = sqlx::query!(
            r#"
                SELECT COUNT(*) as count FROM `authors`;
            "#
        )
        .fetch_one(db_conn_pool)
        .await?;
        Ok(AuthorLength::try_from(records.count)?)
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct AuthorLength(u32);

impl From<AuthorLength> for String {
    fn from(data: AuthorLength) -> Self {
        data.0.to_string()
    }
}

impl TryFrom<i32> for AuthorLength {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}
