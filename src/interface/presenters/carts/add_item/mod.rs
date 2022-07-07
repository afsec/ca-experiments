use crate::{
    domain::entities::cart::CartId,
    interface::{
        presenters::{Endpoint, Presenter},
        repositories::cart::add_item::NewItem,
    },
};

use axum::{http::StatusCode, Json};

use axum::Extension;
use sqlx::SqlitePool;

use super::CartPresenter;

mod model;
mod presenter;
mod view;

pub(super) struct AddItem;
impl Endpoint for AddItem {}

impl CartPresenter {
    pub(crate) async fn add_item(
        Extension(ref sqlite_pool): Extension<SqlitePool>,
        Json(new_cart): Json<NewItem>,
    ) -> Result<Json<CartId>, (StatusCode, String)> {
        AddItem::presenter(&AddItem, sqlite_pool, new_cart).await
    }
}
