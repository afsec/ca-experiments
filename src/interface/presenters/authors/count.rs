use crate::{
    interface::presenters::{Endpoint, Model, Presenter, View, X_TOTAL_COUNT},
    AppResult,
};
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use serde::Serialize;
use sqlx::Sqlite;

use axum::{
    http::{HeaderMap, HeaderValue},
    Extension,
};
use sqlx::SqlitePool;

use super::AuthorPresenter;

pub(super) struct Count;
impl Endpoint for Count {}

impl AuthorPresenter {
    pub(crate) async fn count(
        db_driver: Extension<SqlitePool>,
    ) -> Result<(HeaderMap, ()), (StatusCode, String)> {
        let mut headers = HeaderMap::new();
        // TODO: Implement logic
        let authors_count = "0";
        let header_value = match HeaderValue::from_str(authors_count) {
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

#[async_trait]
impl<'endpoint> Presenter<'endpoint, Count, Sqlite, (), BooksFound, Json<BooksFound>> for Count {}

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, (), BooksFound> for Count {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        submitted_data: (),
    ) -> AppResult<BooksFound> {
        Ok(BooksFound::from(0))
    }
}

#[async_trait]
impl<'endpoint> View<'endpoint, BooksFound, Json<BooksFound>> for Count {
    async fn view(
        &'endpoint self,
        model_result: AppResult<BooksFound>,
    ) -> Result<Json<BooksFound>, (StatusCode, String)> {
        match model_result {
            Ok(books_found) => Ok(Json(books_found)),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct BooksFound(u16);

impl From<u16> for BooksFound {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
