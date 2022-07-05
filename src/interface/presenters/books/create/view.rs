use super::Create;
use crate::domain::entities::book::BookId;
use crate::{interface::presenters::View, AppResult};

use async_trait::async_trait;
use axum::http::StatusCode;
use axum::Json;

#[async_trait]
impl<'endpoint> View<'endpoint, BookId, Json<BookId>> for Create {
    async fn view(
        &'endpoint self,
        model_result: AppResult<BookId>,
    ) -> Result<Json<BookId>, (StatusCode, String)> {
        match model_result {
            Ok(books) => Ok(Json(books)),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}
