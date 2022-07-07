use super::ReadAll;
use crate::interface::repositories::order::read_all::Order;
use crate::{interface::presenters::View, AppResult};

use async_trait::async_trait;
use axum::http::StatusCode;
use axum::Json;

#[async_trait]
impl<'endpoint> View<'endpoint, Vec<Order>, Json<Vec<Order>>> for ReadAll {
    async fn view(
        &'endpoint self,
        model_result: AppResult<Vec<Order>>,
    ) -> Result<Json<Vec<Order>>, (StatusCode, String)> {
        match model_result {
            Ok(orders) => Ok(Json(orders)),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}
