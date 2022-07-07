use super::Count;
use crate::{
    interface::{presenters::View, repositories::cart::count::CartLength},
    AppResult,
};

use async_trait::async_trait;
use axum::http::StatusCode;

#[async_trait]
impl<'endpoint> View<'endpoint, CartLength, String> for Count {
    async fn view(
        &'endpoint self,
        model_result: AppResult<CartLength>,
    ) -> Result<String, (StatusCode, String)> {
        match model_result {
            Ok(carts_length) => Ok(carts_length.into()),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}
