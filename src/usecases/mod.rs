use async_trait::async_trait;

use crate::{domain::entities::DomainEntityValidator, AppResult};

// * Application-specific business logic for our Library app.
pub(crate) mod author;
pub(crate) mod book;
pub(crate) mod publisher;

#[async_trait]
pub(crate) trait UseCaseValidator: DomainEntityValidator {
    async fn validate_usecase(&self) -> AppResult<()>;
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
