use crate::{
    domain::book::{BookId, BookName},
    interface::presenters::{Endpoint, Model, Presenter, View},
    AppResult,
};
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use serde::Serialize;
use sqlx::Sqlite;

pub(super) struct ReadAll;
impl Endpoint for ReadAll {}

#[async_trait]
impl<'endpoint> Presenter<'endpoint, ReadAll, Sqlite, (), Vec<Book>, Json<Vec<Book>>> for ReadAll {}

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, (), Vec<Book>> for ReadAll {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        submitted_data: (),
    ) -> AppResult<Vec<Book>> {
        Ok(vec![])
    }
}

#[async_trait]
impl<'endpoint> View<'endpoint, Vec<Book>, Json<Vec<Book>>> for ReadAll {
    async fn view(
        &'endpoint self,
        model_result: AppResult<Vec<Book>>,
    ) -> Result<Json<Vec<Book>>, (StatusCode, String)> {
        match model_result {
            Ok(books) => Ok(Json(books)),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct Book {
    id: BookId,
    name: BookName,
}
