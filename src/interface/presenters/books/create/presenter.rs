use async_trait::async_trait;
use axum::Json;
use sqlx::Sqlite;

use crate::{
    domain::entities::book::BookId,
    interface::{presenters::Presenter, repositories::book::create::NewBook},
};

use super::Create;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, Create, Sqlite, NewBook, BookId, Json<BookId>> for Create {}
