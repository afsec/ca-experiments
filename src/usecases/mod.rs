/// Service – an object that implements business logic which doesn’t belong in an entity or
///  a value object.
pub(crate) trait Service {}

/// Factory – an object or method that implements object creation logic which is too
/// complex to be done directly by a constructor. A factory might be implemented as a
///  static method of a class.
pub(crate) trait Factory {}

/// Value object – an object which is a collection of values. Two value objects whose
/// attributes have the same values can be used interchangeably. An example of a value
/// object is a Money class, which consists of a currency and an amount.
pub(crate) trait Value {}
