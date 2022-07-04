use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AuthorId(i64);
// TODO: Implement Validator

impl From<AuthorId> for i64 {
    fn from(id: AuthorId) -> Self {
        id.0
    }
}

impl From<i64> for AuthorId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AuthorName(String);

impl From<AuthorName> for String {
    fn from(name: AuthorName) -> Self {
        name.0
    }
}

impl From<String> for AuthorName {
    fn from(value: String) -> Self {
        Self(value)
    }
}
