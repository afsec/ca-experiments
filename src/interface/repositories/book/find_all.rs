use super::BookRepo;
use crate::{
    domain::entities::book::{
        BookAuthor, BookId, BookPrice, BookPublisher, BookQuantity, BookTitle,
    },
    AppResult,
};
use serde::Serialize;
use sqlx::{Database, Pool};

#[derive(Debug, Serialize)]
pub(crate) struct Book {
    id: BookId,
    title: BookTitle,
    author: BookAuthor,       // FK
    publisher: BookPublisher, // FK
    price: BookPrice,
    quantity: BookQuantity,
}

// pub(crate) type BookFromSQLx = (
//     BookId,
//     BookTitle,
//     BookAuthor,    // FK
//     BookPublisher, // FK
//     BookPrice,
//     BookQuantity,
// );

// ? FromRow
// TODO:

impl BookRepo {
    pub(crate) async fn find_all<DBDRIVER>(db_conn_pool: &Pool<DBDRIVER>) -> AppResult<Vec<Book>>
    where
        DBDRIVER: Database,
    {
        
        Ok(vec![])
    }
}
