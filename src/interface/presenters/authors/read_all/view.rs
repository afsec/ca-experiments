use super::ReadAll;
use crate::interface::repositories::author::read_all::Author;
use crate::{interface::presenters::View, AppResult};

use async_trait::async_trait;
use axum::http::StatusCode;
use axum::Json;

#[async_trait]
impl<'endpoint> View<'endpoint, Vec<Author>, Json<Vec<Author>>> for ReadAll {
    async fn view(
        &'endpoint self,
        model_result: AppResult<Vec<Author>>,
    ) -> Result<Json<Vec<Author>>, (StatusCode, String)> {
        match model_result {
            Ok(authors) => Ok(Json(authors)),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}
