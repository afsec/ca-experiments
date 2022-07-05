use async_trait::async_trait;
use axum::Json;
use sqlx::Sqlite;

use crate::interface::{presenters::Presenter, repositories::author::find_all::Author};

use super::Create;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, Create, Sqlite, (), Vec<Author>, Json<Vec<Author>>> for Create {}
