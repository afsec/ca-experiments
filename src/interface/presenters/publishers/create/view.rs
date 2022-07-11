use super::Create;
use crate::domain::entities::publisher::fields::PublisherId;
use crate::{interface::presenters::View, AppResult};

use async_trait::async_trait;
use axum::http::StatusCode;
use axum::Json;

#[async_trait]
impl<'endpoint> View<'endpoint, PublisherId, Json<PublisherId>> for Create {
    async fn view(
        &'endpoint self,
        model_result: AppResult<PublisherId>,
    ) -> Result<Json<PublisherId>, (StatusCode, String)> {
        match model_result {
            Ok(publishers) => Ok(Json(publishers)),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}
