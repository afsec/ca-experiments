use super::Count;
use crate::{
    interface::{presenters::View, repositories::order::count::OrderLength},
    AppResult,
};

use async_trait::async_trait;
use axum::http::StatusCode;

#[async_trait]
impl<'endpoint> View<'endpoint, OrderLength, String> for Count {
    async fn view(
        &'endpoint self,
        model_result: AppResult<OrderLength>,
    ) -> Result<String, (StatusCode, String)> {
        match model_result {
            Ok(orders_length) => Ok(orders_length.into()),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}
