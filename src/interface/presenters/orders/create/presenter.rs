use async_trait::async_trait;
use axum::Json;
use sqlx::Sqlite;

use crate::{
    domain::entities::order::OrderId,
    interface::{presenters::Presenter, repositories::order::create::NewOrder},
};

use super::Create;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, Create, Sqlite, NewOrder, OrderId, Json<OrderId>> for Create {}
