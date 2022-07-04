use crate::{
    domain::book::{BookId, BookName},
    interface::presenters::{Endpoint, Model, Presenter, View},
    AppResult,
};
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use serde::Serialize;
use serde_json::json;
use sqlx::Sqlite;

pub(super) struct Count;
impl Endpoint for Count {}

#[async_trait]
impl<'endpoint> Presenter<'endpoint, Count, Sqlite, (), BooksFound, Json<BooksFound>> for Count {}

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, (), BooksFound> for Count {
    async fn model(
        &'endpoint self,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        submitted_data: (),
    ) -> AppResult<BooksFound> {
        Ok(BooksFound::from(0))
    }
}

#[async_trait]
impl<'endpoint> View<'endpoint, BooksFound, Json<BooksFound>> for Count {
    async fn view(
        &'endpoint self,
        model_result: AppResult<BooksFound>,
    ) -> Result<Json<BooksFound>, (StatusCode, String)> {
        match model_result {
            Ok(books_found) => Ok(Json(books_found)),
            Err(error) => Err((StatusCode::CONFLICT, format!(r#"{{ "error": "{error} }}"#))),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct BooksFound(u16);

impl From<u16> for BooksFound {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
