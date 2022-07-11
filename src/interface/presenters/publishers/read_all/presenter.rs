use async_trait::async_trait;
use axum::Json;
use sqlx::Sqlite;

use crate::{domain::entities::publisher::structs::Publisher, interface::presenters::Presenter};

use super::ReadAll;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, ReadAll, Sqlite, (), Vec<Publisher>, Json<Vec<Publisher>>>
    for ReadAll
{
}
