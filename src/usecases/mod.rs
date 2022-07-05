use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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
    async fn validate_usecase(&self) -> AppResult<()>
    where
        Self: DomainEntity + Sized + Sync;
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct FieldInteractor<T>(T)
where
    T: DomainEntity + UseCase;

impl<T> FieldInteractor<T>
where
    T: DomainEntity + UseCase + Sync,
{
    pub(crate) async fn interact(self) -> AppResult<T> {
        self.0.validate_entity().await?;
        self.0.validate_usecase().await?;
        Ok(self.0)
    }
}
