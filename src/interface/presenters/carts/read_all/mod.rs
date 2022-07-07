use crate::interface::{
    presenters::{Endpoint, Presenter},
    repositories::cart::read_all::Cart,
};

use axum::{http::StatusCode, Json};

use axum::Extension;
use sqlx::SqlitePool;

use super::CartPresenter;

mod model;
mod presenter;
mod view;

pub(super) struct ReadAll;
impl Endpoint for ReadAll {}

impl CartPresenter {
    pub(crate) async fn read_all(
        Extension(ref sqlite_pool): Extension<SqlitePool>,
    ) -> Result<Json<Vec<Cart>>, (StatusCode, String)> {
        ReadAll::presenter(&ReadAll, sqlite_pool, ()).await
    }
}
