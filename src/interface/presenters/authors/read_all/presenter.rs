use async_trait::async_trait;
use axum::Json;
use sqlx::Sqlite;

use crate::interface::{presenters::Presenter, repositories::author::read_all::Author};

use super::ReadAll;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, ReadAll, Sqlite, (), Vec<Author>, Json<Vec<Author>>>
    for ReadAll
{
}
