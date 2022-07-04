use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookId(u32);

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookTitle(String);

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookAuthor(u32);

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookPublisher(u32);

// #[derive(Debug, Serialize)]
// pub(crate) struct Book {
//     id: BookId,
//     title: BookTitle,
//     author: BookAuthor, // FK
//     publisher: BookPublisher, // FK
//     price: u32,
//     quantity: u32,
// }
