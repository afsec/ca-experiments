use async_trait::async_trait;
use axum::Json;
use sqlx::Sqlite;

use crate::{
    domain::entities::publisher::fields::PublisherId,
    interface::{presenters::Presenter, repositories::publisher::create::NewPublisher},
};

use super::Create;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, Create, Sqlite, NewPublisher, PublisherId, Json<PublisherId>>
    for Create
{
}
