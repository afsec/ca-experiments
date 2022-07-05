use crate::interface::{
    presenters::{Endpoint, Presenter},
    repositories::book::read_all::Book,
};

use axum::{http::StatusCode, Json};

use axum::Extension;
use sqlx::SqlitePool;

use super::BookPresenter;

mod model;
mod presenter;
mod view;

pub(super) struct ReadAll;
impl Endpoint for ReadAll {}

impl BookPresenter {
    pub(crate) async fn read_all(
        Extension(ref sqlite_pool): Extension<SqlitePool>,
    ) -> Result<Json<Vec<Book>>, (StatusCode, String)> {
        ReadAll::presenter(&ReadAll, sqlite_pool, ()).await
    }
}
