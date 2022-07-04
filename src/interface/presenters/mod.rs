pub(crate) mod users;


/// Repository – an object that provides access to persistent entities and encapsulates the
///  mechanism for accessing the database.
pub(crate) trait Repository {}

/// Entity – an object with a persistent identity. Two entities whose attributes have the
/// same values are still different objects. In a Java EE application, classes which are
/// persisted using JPA @Entity are usually DDD entities.
pub(crate) trait Entity {}

