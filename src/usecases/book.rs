use async_trait::async_trait;

use crate::{
    domain::entities::book::{BookId, BookTitle},
    AppResult,
};

use super::UseCaseValidator;

// * Id
#[async_trait]
impl UseCaseValidator for BookId {
    async fn validate_usecase(&self) -> AppResult<()> {
        // TODO
        Ok(())
    }
}

// * Name
#[async_trait]
impl UseCaseValidator for BookTitle {
    async fn validate_usecase(&self) -> AppResult<()> {
        // TODO
        Ok(())
    }
}
