use crate::{
    domain::entities::cart::CartId,
    interface::{
        presenters::{Endpoint, Presenter},
        repositories::cart::create::NewCart,
    },
};

use axum::{http::StatusCode, Json};

use axum::Extension;
use sqlx::SqlitePool;

use super::CartPresenter;

mod model;
mod presenter;
mod view;

pub(super) struct Create;
impl Endpoint for Create {}

impl CartPresenter {
    pub(crate) async fn create(
        Extension(ref sqlite_pool): Extension<SqlitePool>,
        Json(new_cart): Json<NewCart>,
    ) -> Result<Json<CartId>, (StatusCode, String)> {
        Create::presenter(&Create, sqlite_pool, new_cart).await
    }
}
