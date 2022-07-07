use crate::{
    domain::entities::order::OrderId,
    interface::{
        presenters::{Endpoint, Presenter},
        repositories::order::create::NewOrder,
    },
};

use axum::{http::StatusCode, Json};

use axum::Extension;
use sqlx::SqlitePool;

use super::OrderPresenter;

mod model;
mod presenter;
mod view;

pub(super) struct Create;
impl Endpoint for Create {}

impl OrderPresenter {
    pub(crate) async fn create(
        Extension(ref sqlite_pool): Extension<SqlitePool>,
        Json(new_order): Json<NewOrder>,
    ) -> Result<Json<OrderId>, (StatusCode, String)> {
        Create::presenter(&Create, sqlite_pool, new_order).await
    }
}
