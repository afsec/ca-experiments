use super::ReadAll;
use crate::domain::entities::publisher::structs::Publisher;
use crate::{interface::presenters::View, AppResult};

use async_trait::async_trait;
use axum::http::StatusCode;
use axum::Json;

#[async_trait]
impl<'endpoint> View<'endpoint, Vec<Publisher>, Json<Vec<Publisher>>> for ReadAll {
    async fn view(
        &'endpoint self,
        model_result: AppResult<Vec<Publisher>>,
    ) -> Result<Json<Vec<Publisher>>, (StatusCode, String)> {
        match model_result {
            Ok(publishers) => Ok(Json(publishers)),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}
