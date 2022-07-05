use super::BookRepo;
use crate::{
    domain::entities::book::{BookId, BookTitle},
    AppResult,
};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

// * DepartmentFromSqlx
#[derive(Debug, FromRow)]
pub(crate) struct BookFromSQLx {
    id: i64,
    title: String,
}

impl TryFrom<Book> for BookFromSQLx {
    type Error = anyhow::Error;

    fn try_from(book: Book) -> Result<Self, Self::Error> {
        Ok(Self {
            id: book.id.try_into()?,
            title: book.title.try_into()?,
        })
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct Book {
    id: BookId,
    title: BookTitle,
}

impl TryFrom<BookFromSQLx> for Book {
    type Error = anyhow::Error;

    fn try_from(book: BookFromSQLx) -> Result<Self, Self::Error> {
        Ok(Self {
            id: book.id.try_into()?,
            title: book.title.try_into()?,
        })
    }
}

impl BookRepo {
    pub(crate) async fn count(db_conn_pool: &SqlitePool) -> AppResult<BookLength> {
        let records = sqlx::query!(
            r#"
                SELECT COUNT(*) as count FROM `books`;
            "#
        )
        .fetch_one(db_conn_pool)
        .await?;
        Ok(BookLength::try_from(records.count)?)
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct BookLength(u32);

impl From<BookLength> for String {
    fn from(data: BookLength) -> Self {
        data.0.to_string()
    }
}

impl TryFrom<i32> for BookLength {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}
