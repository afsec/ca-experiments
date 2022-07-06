use std::fmt;

use super::BookRepo;
use crate::{
    domain::entities::book::{
        BookAuthor, BookId, BookPrice, BookPublisher, BookQuantity, BookTitle,
    },
    AppResult,
};
use serde::{de, Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct NewBook {
    pub(crate) title: BookTitle,
    #[serde(deserialize_with = "price_from_f64")]
    pub(crate) price: BookPrice,
    pub(crate) quantity: BookQuantity,
    pub(crate) author: BookAuthor,
    pub(crate) publisher: BookPublisher,
}

impl BookRepo {
    pub(crate) async fn create(db_conn_pool: &SqlitePool, new_book: NewBook) -> AppResult<BookId> {
        let rowid = sqlx::query!(
            r#"
                INSERT INTO `books` ("title","price","quantity","author","publisher")
                VALUES (?1,?2,?3,?4,?5);
            "#,
            *new_book.title,
            *new_book.price,
            *new_book.quantity,
            *new_book.author,
            *new_book.publisher
        )
        .execute(db_conn_pool)
        .await?
        .last_insert_rowid();
        Ok(rowid.try_into()?)
    }
}

/// Deserialize f64 to `BookPrice`
pub(in crate) fn price_from_f64<'de, D>(d: D) -> Result<BookPrice, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(
        BookPrice::try_from(d.deserialize_f64(BookPriceVisitor)?).map_err(|error| {
            tracing::error!("{error:?}");
            serde::de::Error::custom(format!("Can't deserizalize f64. Reason: [{error}]"))
        })?,
    )
}
// Visitor to help deserialize f64 to `BookPrice`
struct BookPriceVisitor;

impl<'de> de::Visitor<'de> for BookPriceVisitor {
    type Value = f64;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a price represents a f64")
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v)
    }
}
