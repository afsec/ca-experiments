use std::ops::Deref;

use serde::{Deserialize, Serialize};

// * Id
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AuthorId(u32);

// TODO: Implement Validator

impl From<AuthorId> for u32 {
    fn from(id: AuthorId) -> Self {
        id.0
    }
}

impl From<u32> for AuthorId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl TryFrom<i64> for AuthorId {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}

impl TryFrom<AuthorId> for i64 {
    type Error = anyhow::Error;

    fn try_from(data: AuthorId) -> Result<Self, Self::Error> {
        Ok(Self::from(data.0))
    }
}

// * Name
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AuthorName(String);

impl Deref for AuthorName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
