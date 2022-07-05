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

impl BookRepo {
    pub(crate) async fn read_all<DBDRIVER>(db_conn_pool: &Pool<DBDRIVER>) -> AppResult<Vec<Book>>
    where
        DBDRIVER: Database,
    {
        
        Ok(vec![])
    }
}
