use crate::interface::{presenters::Presenter, repositories::cart::count::CartLength};
use async_trait::async_trait;
use sqlx::Sqlite;

use super::Count;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, Count, Sqlite, (), CartLength, String> for Count {}
