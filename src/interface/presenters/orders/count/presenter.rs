use crate::interface::{presenters::Presenter, repositories::order::count::OrderLength};
use async_trait::async_trait;
use sqlx::Sqlite;

use super::Count;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, Count, Sqlite, (), OrderLength, String> for Count {}
