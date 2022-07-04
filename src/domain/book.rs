use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct Book {
    id: BookId,
    name: BookName,
}

#[derive(Debug, Serialize)]
pub(crate) struct BookId(u16);

#[derive(Debug, Serialize)]
pub(crate) struct BookName(String);

