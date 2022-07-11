use async_trait::async_trait;
use axum::Json;
use sqlx::Sqlite;

use crate::{
    domain::entities::cart::CartId,
    interface::{presenters::Presenter, repositories::cart::create::NewCart},
};

use super::Create;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, Create, Sqlite, NewCart, CartId, Json<CartId>> for Create {}
