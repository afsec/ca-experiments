use super::BookRepo;
use crate::{
    domain::entities::book::{
        BookAuthor, BookId, BookPrice, BookPublisher, BookQuantity, BookTitle,
    },
    AppResult,
};
use serde::{Serialize, Serializer};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, FromRow)]
pub(crate) struct BookFromSQLx {
    id: i64,
    title: String,
    price: i64,
    quantity: i64,
    author: i64,
    publisher: i64,
}

#[derive(Debug, Serialize)]
pub(crate) struct Book {
    id: BookId,
    title: BookTitle,
    #[serde(serialize_with = "price_to_f64")]
    price: BookPrice,
    quantity: BookQuantity,
    author: BookAuthor,
    publisher: BookPublisher,
}

impl TryFrom<BookFromSQLx> for Book {
    type Error = anyhow::Error;

    fn try_from(book: BookFromSQLx) -> Result<Self, Self::Error> {
        Ok(Self {
            id: book.id.try_into()?,
            title: book.title.try_into()?,
            price: book.price.try_into()?,
            quantity: book.quantity.try_into()?,
            author: book.author.try_into()?,
            publisher: book.publisher.try_into()?,
        })
    }
}

impl BookRepo {
    pub(crate) async fn read_all(db_conn_pool: &SqlitePool) -> AppResult<Vec<Book>> {
        let records: Vec<BookFromSQLx> = sqlx::query_as!(
            BookFromSQLx,
            r#"
            SELECT id, title, price, quantity, author, publisher
            FROM `books`;
        "#,
        )
        .fetch_all(db_conn_pool)
        .await?;

        // * To improve performance -> https://github.com/launchbadge/sqlx/issues/117

        let books: AppResult<Vec<Book>> = records
            .into_iter()
            .map(|record| Ok(record.try_into()?))
            .collect();
        tracing::debug!("Books: {:?}", &books);
        Ok(books?)
    }
}

fn price_to_f64<S>(book_price: &BookPrice, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let price = book_price
        .try_into()
        .map_err(|error| {
            tracing::error!("{error} on file [{}] at line [{}]", file!(), line!());
            error
        })
        .unwrap_or_default();
    s.serialize_f64(price)
}
