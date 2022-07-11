use async_trait::async_trait;
use axum::Json;
use sqlx::Sqlite;

use crate::{
    domain::entities::cart::CartId,
    interface::{presenters::Presenter, repositories::cart::add_item::NewItem},
};

use super::AddItem;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, AddItem, Sqlite, NewItem, CartId, Json<CartId>> for AddItem {}
