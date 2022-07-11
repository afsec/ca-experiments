use async_trait::async_trait;
use axum::Json;
use sqlx::Sqlite;

use crate::interface::{presenters::Presenter, repositories::cart::read_all::Cart};

use super::ReadAll;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, ReadAll, Sqlite, (), Vec<Cart>, Json<Vec<Cart>>> for ReadAll {}
