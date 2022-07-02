use crate::helper::endpoint::Endpoint;
use async_trait::async_trait;
use axum::{http::StatusCode, Extension};
use sqlx::SqlitePool;

pub(crate) struct Users;

#[async_trait]
impl Endpoint<String, Result<(StatusCode, String), (StatusCode, String)>> for Users {
    async fn count(
        Extension(ref sqlite_pool): Extension<SqlitePool>,
    ) -> Result<(StatusCode, String), (StatusCode, String)> {
        Ok((StatusCode::OK, "".to_string()))
    }
}
