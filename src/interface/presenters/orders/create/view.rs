use super::Create;
use crate::domain::entities::order::OrderId;
use crate::{interface::presenters::View, AppResult};

use async_trait::async_trait;
use axum::http::StatusCode;
use axum::Json;

#[async_trait]
impl<'endpoint> View<'endpoint, OrderId, Json<OrderId>> for Create {
    async fn view(
        &'endpoint self,
        model_result: AppResult<OrderId>,
    ) -> Result<Json<OrderId>, (StatusCode, String)> {
        match model_result {
            Ok(orders) => Ok(Json(orders)),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}
