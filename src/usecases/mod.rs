use async_trait::async_trait;
use serde::Deserialize;

use crate::{
    domain::{entities::DomainEntityValidator, DataValidator},
    AppResult,
};

/// Application-specific business logic for our Library app.
// Save Author in our library
// Save Book in our library
// Fetch all books from our library
pub(crate) mod author;
pub(crate) mod book;
pub(crate) mod publisher;

#[async_trait]
pub(crate) trait UseCaseValidator: DomainEntityValidator {
    async fn validate_usecase(&self) -> AppResult<()>
    where
        Self: Sync;
}

// TODO: Implement macro derive `NoUseCaseValidator` with the follow implementation:

/* ```
#[async_trait]
impl UseCaseValidator for $sometype {
    async fn validate_usecase(&self) -> AppResult<()> {
        Ok(())
    }
}/// ```
*/

#[derive(Debug, Deserialize)]
pub(crate) struct FieldInteractor<T>(T)
where
    T: DataValidator + DomainEntityValidator + UseCaseValidator;

impl<T> FieldInteractor<T>
where
    T: DataValidator + DomainEntityValidator + UseCaseValidator + Sync,
{
    pub(crate) async fn interact(self) -> AppResult<T> {
        self.0.validate_data().await?;
        self.0.validate_entity().await?;
        self.0.validate_usecase().await?;
        Ok(self.0)
    }
}
