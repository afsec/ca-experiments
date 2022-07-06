use crate::interface::{presenters::Presenter, repositories::publisher::count::PublisherLength};
use async_trait::async_trait;
use sqlx::Sqlite;

use super::Count;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, Count, Sqlite, (), PublisherLength, String> for Count {}
