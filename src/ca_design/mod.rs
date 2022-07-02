pub(crate) mod entities;
pub(crate) mod factories;
pub(crate) mod repositories;
pub(crate) mod services;
pub(crate) mod values;

/// Entity – an object with a persistent identity. Two entities whose attributes have the
/// same values are still different objects. In a Java EE application, classes which are
/// persisted using JPA @Entity are usually DDD entities.
pub(crate) trait Entity {}

/// Value object – an object which is a collection of values. Two value objects whose
/// attributes have the same values can be used interchangeably. An example of a value
/// object is a Money class, which consists of a currency and an amount.
pub(crate) trait Value {}

/// Factory – an object or method that implements object creation logic which is too
/// complex to be done directly by a constructor. A factory might be implemented as a
///  static method of a class.
pub(crate) trait Factory {}

/// Repository – an object that provides access to persistent entities and encapsulates the
///  mechanism for accessing the database.
pub(crate) trait Repository {}

/// Service – an object that implements business logic which doesn’t belong in an entity or
///  a value object.
pub(crate) trait Service {}
