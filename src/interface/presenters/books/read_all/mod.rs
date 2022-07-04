use crate::{
    domain::entities::book::{
        BookAuthor, BookId, BookPrice, BookPublisher, BookQuantity, BookTitle,
    },
    interface::presenters::Endpoint,
};

use serde::Serialize;

mod model;
mod presenter;
mod view;

pub(super) struct ReadAll;
impl Endpoint for ReadAll {}

// #[derive(Debug, Serialize)]
// struct Book {
//     id: BookId,
//     title: BookTitle,
//     author: BookAuthor,       // FK
//     publisher: BookPublisher, // FK
//     price: BookPrice,
//     quantity: BookQuantity,
// }
