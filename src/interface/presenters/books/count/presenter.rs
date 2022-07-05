use async_trait::async_trait;
use sqlx::Sqlite;
use crate::interface::{presenters::Presenter, repositories::book::count::BookLength};

use super::Count;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, Count, Sqlite, (), BookLength, String> for Count {}
