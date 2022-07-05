use crate::interface::{
    presenters::{Endpoint, Presenter},
    repositories::publisher::read_all::Publisher,
};

use axum::{http::StatusCode, Json};

use axum::Extension;
use sqlx::SqlitePool;

use super::PublisherPresenter;

mod model;
mod presenter;
mod view;

pub(super) struct ReadAll;
impl Endpoint for ReadAll {}

impl PublisherPresenter {
    pub(crate) async fn read_all(
        Extension(ref sqlite_pool): Extension<SqlitePool>,
    ) -> Result<Json<Vec<Publisher>>, (StatusCode, String)> {
        ReadAll::presenter(&ReadAll, sqlite_pool, ()).await
    }
}
