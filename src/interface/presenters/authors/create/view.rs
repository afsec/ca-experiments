use super::Create;
use crate::interface::repositories::author::find_all::Author;
use crate::{interface::presenters::View, AppResult};

use async_trait::async_trait;
use axum::http::StatusCode;
use axum::Json;

#[async_trait]
impl<'endpoint> View<'endpoint, Vec<Author>, Json<Vec<Author>>> for Create {
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
