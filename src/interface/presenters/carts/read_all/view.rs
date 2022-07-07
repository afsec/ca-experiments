use super::ReadAll;
use crate::interface::repositories::cart::read_all::Cart;
use crate::{interface::presenters::View, AppResult};

use async_trait::async_trait;
use axum::http::StatusCode;
use axum::Json;

#[async_trait]
impl<'endpoint> View<'endpoint, Vec<Cart>, Json<Vec<Cart>>> for ReadAll {
    async fn view(
        &'endpoint self,
        model_result: AppResult<Vec<Cart>>,
    ) -> Result<Json<Vec<Cart>>, (StatusCode, String)> {
        match model_result {
            Ok(carts) => Ok(Json(carts)),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}
