use async_trait::async_trait;
use sqlx::Sqlite;
use crate::interface::{presenters::Presenter, repositories::author::count::AuthorLength};

use super::Count;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, Count, Sqlite, (), AuthorLength, String> for Count {}
