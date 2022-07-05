use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookId(u32);

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookTitle(String);

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookPrice(u32);

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookQuantity(u32);

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookAuthor(u32);

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookPublisher(u32);
