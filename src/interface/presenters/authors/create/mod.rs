use crate::{
    domain::entities::author::AuthorId,
    interface::presenters::{Endpoint, Presenter},
    usecases::author::NewAuthor,
};

use axum::{http::StatusCode, Json};

use axum::Extension;
use sqlx::SqlitePool;

use super::AuthorPresenter;

mod model;
mod presenter;
mod view;

pub(super) struct Create;
impl Endpoint for Create {}

impl AuthorPresenter {
    pub(crate) async fn create(
        Extension(ref sqlite_pool): Extension<SqlitePool>,
        Json(new_author): Json<NewAuthor>,
    ) -> Result<Json<AuthorId>, (StatusCode, String)> {
        Create::presenter(&Create, sqlite_pool, new_author).await
    }
}
