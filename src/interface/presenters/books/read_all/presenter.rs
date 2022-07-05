use async_trait::async_trait;
use axum::Json;
use sqlx::Sqlite;

use crate::interface::{presenters::Presenter, repositories::book::read_all::Book};

use super::ReadAll;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, ReadAll, Sqlite, (), Vec<Book>, Json<Vec<Book>>> for ReadAll {}
