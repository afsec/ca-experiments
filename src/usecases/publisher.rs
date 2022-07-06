use async_trait::async_trait;

use crate::{
    domain::entities::publisher::{PublisherId, PublisherName},
    AppResult,
};

use super::UseCaseValidator;

// * Id

#[async_trait]
impl UseCaseValidator for PublisherId {
    async fn validate_usecase(&self) -> AppResult<()> {
        Ok(())
    }
}

// * Name

#[async_trait]
impl UseCaseValidator for PublisherName {
    async fn validate_usecase(&self) -> AppResult<()> {
        Ok(())
    }
}
