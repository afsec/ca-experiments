use async_trait::async_trait;
use axum::Json;
use sqlx::Sqlite;

use crate::{
    domain::entities::author::AuthorId, interface::presenters::Presenter,
    usecases::author::NewAuthor,
};

use super::Create;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, Create, Sqlite, NewAuthor, AuthorId, Json<AuthorId>>
    for Create
{
}
