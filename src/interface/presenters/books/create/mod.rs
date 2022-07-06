use crate::{
    domain::entities::book::BookId,
    interface::{
        presenters::{Endpoint, Presenter},
        repositories::book::create::NewBook,
    },
};

use axum::{http::StatusCode, Json};

use axum::Extension;
use sqlx::SqlitePool;

use super::BookPresenter;

mod model;
mod presenter;
mod view;

pub(super) struct Create;
impl Endpoint for Create {}

impl BookPresenter {
    pub(crate) async fn create(
        Extension(ref sqlite_pool): Extension<SqlitePool>,
        Json(new_book): Json<NewBook>,
    ) -> Result<Json<BookId>, (StatusCode, String)> {
        Create::presenter(&Create, sqlite_pool, new_book).await
    }
}
