use crate::interface::{
    presenters::{Endpoint, Presenter},
    repositories::author::read_all::Author,
};

use axum::{http::StatusCode, Json};

use axum::Extension;
use sqlx::SqlitePool;

use super::AuthorPresenter;

mod model;
mod presenter;
mod view;

pub(super) struct ReadAll;
impl Endpoint for ReadAll {}

impl AuthorPresenter {
    pub(crate) async fn read_all(
        Extension(ref sqlite_pool): Extension<SqlitePool>,
    ) -> Result<Json<Vec<Author>>, (StatusCode, String)> {
        ReadAll::presenter(&ReadAll, sqlite_pool, ()).await
    }
}
