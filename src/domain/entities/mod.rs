use async_trait::async_trait;

use crate::AppResult;

pub(crate) mod author;
pub(crate) mod book;
pub(crate) mod publisher;

/// Entity – an object with a persistent identity. Two entities whose attributes have the
/// same values are still different objects. In a Java EE application, classes which are
/// persisted using JPA @Entity are usually DDD entities.
#[async_trait]
pub(crate) trait DomainEntityValidator {
    async fn validate_entity(&self) -> AppResult<()>
    where
        Self: Sync;
}

// TODO: Implement macro derive `NoDomainEntityValidator` with the follow implementation:

/* ```
/// #[async_trait]
/// impl DomainEntityValidator for $sometyspe {
///     async fn validate_entity(&self) -> AppResult<()> {
///         Ok(())
///     }
/// }
/// ```
*/
