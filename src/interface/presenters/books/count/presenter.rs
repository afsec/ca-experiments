use crate::interface::{presenters::Presenter, repositories::book::count::BookLength};
use async_trait::async_trait;
use sqlx::Sqlite;

use super::Count;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, Count, Sqlite, (), BookLength, String> for Count {}
