use super::Count;
use crate::{
    interface::{presenters::View, repositories::author::count::AuthorLength},
    AppResult,
};

use async_trait::async_trait;
use axum::http::StatusCode;

#[async_trait]
impl<'endpoint> View<'endpoint, AuthorLength, String> for Count {
    async fn view(
        &'endpoint self,
        model_result: AppResult<AuthorLength>,
    ) -> Result<String, (StatusCode, String)> {
        match model_result {
            Ok(authors_length) => Ok(authors_length.into()),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}
