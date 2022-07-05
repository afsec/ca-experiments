use std::ops::Deref;

use serde::{Deserialize, Serialize};

// * Id
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookId(u32);

// TODO: Implement Validator

impl From<BookId> for u32 {
    fn from(id: BookId) -> Self {
        id.0
    }
}

impl From<u32> for BookId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl TryFrom<i64> for BookId {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}

impl TryFrom<BookId> for i64 {
    type Error = anyhow::Error;

    fn try_from(data: BookId) -> Result<Self, Self::Error> {
        Ok(Self::from(data.0))
    }
}

// * Title
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookTitle(String);

impl Deref for BookTitle {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<BookTitle> for String {
    fn from(name: BookTitle) -> Self {
        name.0
    }
}

impl From<String> for BookTitle {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct BookPrice(u32);

impl Deref for BookPrice {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<i64> for BookPrice {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}

impl TryInto<f64> for &BookPrice {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<f64, Self::Error> {
        Ok(f64::try_from(self.0)? / 100.0)
    }
}

impl TryFrom<f64> for BookPrice {
    type Error = anyhow::Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let number = u32::try_from((value * 100.0).floor() as u64)?;
        Ok(Self(number))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookQuantity(u32);

impl Deref for BookQuantity {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<i64> for BookQuantity {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookAuthor(u32);

impl Deref for BookAuthor {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<i64> for BookAuthor {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BookPublisher(u32);

impl Deref for BookPublisher {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<i64> for BookPublisher {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}
