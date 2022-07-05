use async_trait::async_trait;

use crate::{
    domain::entities::publisher::{PublisherId, PublisherName},
    AppResult,
};

use super::UseCase;

// * Id

#[async_trait]
impl UseCase for PublisherId {
    async fn validate_usecase(&self) -> AppResult<()> {
        Ok(())
    }
}

// * Name

#[async_trait]
impl UseCase for PublisherName {
    async fn validate_usecase(&self) -> AppResult<()> {
        Ok(())
    }
}
