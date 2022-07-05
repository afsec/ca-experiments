use async_trait::async_trait;

use crate::{
    domain::entities::book::{BookId, BookTitle},
    AppResult,
};

use super::{UseCase, Interactor};

// * Id

#[async_trait]
impl UseCase for BookId {
    async fn validate_usecase(self) -> AppResult<Interactor<BookId>> {
        Ok(Interactor(self))
    }
}

// * Name

#[async_trait]
impl UseCase for BookTitle {
    async fn validate_usecase(self) -> AppResult<Interactor<BookTitle>> {
        Ok(Interactor(self))
    }
}
