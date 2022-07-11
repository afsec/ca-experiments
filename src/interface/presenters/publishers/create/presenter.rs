use async_trait::async_trait;
use axum::Json;
use sqlx::Sqlite;

use crate::{
    interface::{presenters::Presenter, repositories::publisher::create::NewPublisher}, domain::entities::publisher::fields::PublisherId,
};

use super::Create;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, Create, Sqlite, NewPublisher, PublisherId, Json<PublisherId>>
    for Create
{
}
