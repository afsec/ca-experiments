use std::ops::Deref;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::AppResult;

use super::DomainEntity;

// * Id
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct PublisherId(u32);

// TODO: Implement Validator

impl From<PublisherId> for u32 {
    fn from(id: PublisherId) -> Self {
        id.0
    }
}

impl From<u32> for PublisherId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl TryFrom<i64> for PublisherId {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}

impl TryFrom<PublisherId> for i64 {
    type Error = anyhow::Error;

    fn try_from(data: PublisherId) -> Result<Self, Self::Error> {
        Ok(Self::from(data.0))
    }
}

#[async_trait]
impl DomainEntity for PublisherId {
    async fn validate_entity(&self) -> AppResult<()> {
        Ok(())
    }
}

// * Name
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct PublisherName(String);

impl Deref for PublisherName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<PublisherName> for String {
    fn from(name: PublisherName) -> Self {
        name.0
    }
}

impl From<String> for PublisherName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[async_trait]
impl DomainEntity for PublisherName {
    async fn validate_entity(&self) -> AppResult<()> {
        Ok(())
    }
}