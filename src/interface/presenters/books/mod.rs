mod count;
mod read_all;

use crate::interface::repositories::book::read_all::Book;

use super::{Presenter, X_TOTAL_COUNT};

use self::read_all::ReadAll;


use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    Extension, Json,
};
use sqlx::SqlitePool;

pub(crate) struct BookPresenter;

impl BookPresenter {
    pub(crate) async fn read_all(
        Extension(ref sqlite_pool): Extension<SqlitePool>,
    ) -> Result<Json<Vec<Book>>, (StatusCode, String)> {
        self::read_all::ReadAll::presenter(&ReadAll, sqlite_pool, ()).await
    }

    pub(crate) async fn count(
        db_driver: Extension<SqlitePool>,
    ) -> Result<(HeaderMap, ()), (StatusCode, String)> {
        let mut headers = HeaderMap::new();
        // TODO: Implement logic
        let books_count = "0";
        let header_value = match HeaderValue::from_str(books_count) {
            Ok(header_value) => header_value,
            Err(error) => {
                // TODO: Implement Error matching
                tracing::warn!("Handler error: {}", error.to_string());
                return Err((
                    StatusCode::CONFLICT,
                    "Error on serialization counted Books.".to_string(),
                ));
            }
        };
        headers.insert(X_TOTAL_COUNT, header_value);

        Ok((headers, ()))
    }
}
