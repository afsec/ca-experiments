use super::Create;
use crate::domain::entities::author::AuthorId;
use crate::{interface::presenters::View, AppResult};

use async_trait::async_trait;
use axum::http::StatusCode;
use axum::Json;

#[async_trait]
impl<'endpoint> View<'endpoint, AuthorId, Json<AuthorId>> for Create {
    async fn view(
        &'endpoint self,
        model_result: AppResult<AuthorId>,
    ) -> Result<Json<AuthorId>, (StatusCode, String)> {
        match model_result {
            Ok(authors) => Ok(Json(authors)),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}
