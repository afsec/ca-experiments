pub(crate) mod user;


/// Entity – an object with a persistent identity. Two entities whose attributes have the
/// same values are still different objects. In a Java EE application, classes which are
/// persisted using JPA @Entity are usually DDD entities.
pub(crate) trait Entity {}

