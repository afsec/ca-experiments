use async_trait::async_trait;

use crate::{
    domain::entities::publisher::{PublisherId, PublisherName},
    AppResult,
};

use super::{Interactor, UseCase};

// * Id

#[async_trait]
impl UseCase for PublisherId {
    async fn validate_usecase(self) -> AppResult<Interactor<PublisherId>> {
        Ok(Interactor(self))
    }
}

// * Name

#[async_trait]
impl UseCase for PublisherName {
    async fn validate_usecase(self) -> AppResult<Interactor<PublisherName>> {
        Ok(Interactor(self))
    }
}
