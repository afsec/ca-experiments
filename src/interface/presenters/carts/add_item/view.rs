use super::AddItem;
use crate::domain::entities::cart::CartId;
use crate::{interface::presenters::View, AppResult};

use async_trait::async_trait;
use axum::http::StatusCode;
use axum::Json;

#[async_trait]
impl<'endpoint> View<'endpoint, CartId, Json<CartId>> for AddItem {
    async fn view(
        &'endpoint self,
        model_result: AppResult<CartId>,
    ) -> Result<Json<CartId>, (StatusCode, String)> {
        match model_result {
            Ok(carts) => Ok(Json(carts)),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}
