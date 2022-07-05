use super::Count;
use crate::{
    interface::{presenters::View, repositories::publisher::count::PublisherLength},
    AppResult,
};

use async_trait::async_trait;
use axum::http::StatusCode;

#[async_trait]
impl<'endpoint> View<'endpoint, PublisherLength, String> for Count {
    async fn view(
        &'endpoint self,
        model_result: AppResult<PublisherLength>,
    ) -> Result<String, (StatusCode, String)> {
        match model_result {
            Ok(publishers_length) => Ok(publishers_length.into()),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}
