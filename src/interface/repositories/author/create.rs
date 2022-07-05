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
    pub(crate) async fn create(db_conn_pool: &SqlitePool, new_author: NewAuthor) -> AppResult<AuthorId> {
        let new_department_oid = DepartmentOid::new(option_oid_pool).await?;
        let name = &(*new_department.name);
        let created_at: NaiveDateTime = DepartmentCreatedAt::now().into();
        let updated_at = &created_at;
        let _ = sqlx::query!(
            r#"
                INSERT INTO `departments` ("oid","name","created_at","updated_at","is_deleted")
                VALUES (?1,?2,?3,?4,0);
            "#,
            *new_department_oid,
            name,
            created_at,
            updated_at
        )
        .execute(db_conn_pool)
        .await?;
    }
}
