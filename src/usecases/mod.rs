use async_trait::async_trait;

use crate::{domain::entities::DomainEntity, AppResult};

/// Application-specific business logic for our Library app.
// Save Author in our library
// Save Book in our library
// Fetch all books from our library
pub(crate) mod author;
pub(crate) mod book;
pub(crate) mod publisher;

#[async_trait]
pub(crate) trait UseCase: DomainEntity {
    async fn validate_usecase(self) -> AppResult<Interactor<Self>>
    where
        Self: DomainEntity + UseCase + Sized + Sync;
}

#[derive(Debug)]
pub(crate) struct Interactor<T>(T)
where
    T: DomainEntity + UseCase;

impl<T> Interactor<T>
where
    T: DomainEntity + UseCase,
{
    async fn new(data: T) -> Self {
        Self(data)
    }
}
