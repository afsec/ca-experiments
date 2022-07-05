use super::ReadAll;
use crate::interface::repositories::book::read_all::Book;
use crate::{interface::presenters::View, AppResult};

use async_trait::async_trait;
use axum::http::StatusCode;
use axum::Json;

#[async_trait]
impl<'endpoint> View<'endpoint, Vec<Book>, Json<Vec<Book>>> for ReadAll {
    async fn view(
        &'endpoint self,
        model_result: AppResult<Vec<Book>>,
    ) -> Result<Json<Vec<Book>>, (StatusCode, String)> {
        match model_result {
            Ok(books) => Ok(Json(books)),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}
