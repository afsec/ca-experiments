/// Domain driven design - https://en.wikipedia.org/wiki/Domain-driven_design
///

/// Core non application-specific business logic.
/// The domain layer is the innermost layer and according to the dependency
/// rule it should be independent of everything
pub(crate) mod entities;

use async_trait::async_trait;

use crate::AppResult;

#[async_trait]
pub(crate) trait DataValidator {
    async fn validate_data(&self) -> AppResult<()>;
}

/// Factory – an object or method that implements object creation logic which is too
/// complex to be done directly by a constructor. A factory might be implemented as a
///  static method of a class.
pub(crate) trait Factory {}

/// Value object – an object which is a collection of values. Two value objects whose
/// attributes have the same values can be used interchangeably. An example of a value
/// object is a Money class, which consists of a currency and an amount.
pub(crate) trait Value {}
