use crate::{
    domain::entities::publisher::fields::PublisherId,
    interface::{
        presenters::{Endpoint, Presenter},
        repositories::publisher::create::NewPublisher,
    },
};

use axum::{http::StatusCode, Json};

use axum::Extension;
use sqlx::SqlitePool;

use super::PublisherPresenter;

mod model;
mod presenter;
mod view;

pub(super) struct Create;
impl Endpoint for Create {}

impl PublisherPresenter {
    pub(crate) async fn create(
        Extension(ref sqlite_pool): Extension<SqlitePool>,
        Json(new_publisher): Json<NewPublisher>,
    ) -> Result<Json<PublisherId>, (StatusCode, String)> {
        Create::presenter(&Create, sqlite_pool, new_publisher).await
    }
}
