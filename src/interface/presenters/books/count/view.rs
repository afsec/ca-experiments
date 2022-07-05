use super::Count;
use crate::{
    interface::{presenters::View, repositories::book::count::BookLength},
    AppResult,
};

use async_trait::async_trait;
use axum::http::StatusCode;

#[async_trait]
impl<'endpoint> View<'endpoint, BookLength, String> for Count {
    async fn view(
        &'endpoint self,
        model_result: AppResult<BookLength>,
    ) -> Result<String, (StatusCode, String)> {
        match model_result {
            Ok(books_length) => Ok(books_length.into()),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}
