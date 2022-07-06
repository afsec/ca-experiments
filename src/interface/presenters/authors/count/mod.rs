use crate::interface::presenters::{Endpoint, Presenter, X_TOTAL_COUNT};

use axum::http::StatusCode;

use axum::{
    http::{HeaderMap, HeaderValue},
    Extension,
};

use sqlx::SqlitePool;

use super::AuthorPresenter;

mod model;
mod presenter;
mod view;

pub(super) struct Count;
impl Endpoint for Count {}

impl AuthorPresenter {
    pub(crate) async fn count(
        Extension(ref sqlite_pool): Extension<SqlitePool>,
    ) -> Result<(HeaderMap, ()), (StatusCode, String)> {
        let mut headers = HeaderMap::new();
        // TODO: Implement logic
        let authors_count = Count::presenter(&Count, sqlite_pool, ()).await?;
        let header_value = match HeaderValue::from_str(&authors_count) {
            Ok(header_value) => header_value,
            Err(error) => {
                // TODO: Implement Error matching
                tracing::warn!("Handler error: {}", error.to_string());
                return Err((
                    StatusCode::CONFLICT,
                    "Error on serialization counted Authors.".to_string(),
                ));
            }
        };
        headers.insert(X_TOTAL_COUNT, header_value);

        Ok((headers, ()))
    }
}
