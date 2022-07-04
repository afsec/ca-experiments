use crate::{
    domain::entities::book::{BookId, BookTitle},
    interface::presenters::Endpoint,
};

use serde::Serialize;

mod model;
mod presenter;
mod view;

pub(super) struct ReadAll;
impl Endpoint for ReadAll {}

#[derive(Debug, Serialize)]
pub(crate) struct Book {
    id: BookId,
    title: BookTitle,
}
