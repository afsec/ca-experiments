use async_trait::async_trait;

use crate::{
    domain::entities::author::{AuthorId, AuthorName},
    AppResult,
};

use super::{Interactor, UseCase};

// * Id

#[async_trait]
impl UseCase for AuthorId {
    async fn validate_usecase(self) -> AppResult<Interactor<AuthorId>> {
        Ok(Interactor(self))
    }
}

// * Name

#[async_trait]
impl UseCase for AuthorName {
    async fn validate_usecase(self) -> AppResult<Interactor<AuthorName>> {
        Ok(Interactor(self))
    }
}
