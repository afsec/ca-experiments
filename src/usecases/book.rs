use async_trait::async_trait;

use crate::{
    domain::entities::book::{BookId, BookTitle},
    AppResult,
};

use super::UseCase;

// * Id

#[async_trait]
impl UseCase for BookId {
    async fn validate_usecase(&self) -> AppResult<()> {
        Ok(())
    }
}

// * Name

#[async_trait]
impl UseCase for BookTitle {
    async fn validate_usecase(&self) -> AppResult<()> {
        Ok(())
    }
}
