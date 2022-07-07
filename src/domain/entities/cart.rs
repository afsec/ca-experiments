use std::ops::Deref;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{domain::DataValidator, AppResult};

use super::DomainEntityValidator;

// * Id
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct CartId(u32);

impl Deref for CartId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: Implement Validator

impl From<CartId> for u32 {
    fn from(id: CartId) -> Self {
        id.0
    }
}

impl From<u32> for CartId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl TryFrom<i64> for CartId {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}

impl TryFrom<CartId> for i64 {
    type Error = anyhow::Error;

    fn try_from(data: CartId) -> Result<Self, Self::Error> {
        Ok(Self::from(data.0))
    }
}

#[async_trait]
impl DataValidator for CartId {
    async fn validate_data(&self) -> AppResult<()> {
        Ok(())
    }
}

#[async_trait]
impl DomainEntityValidator for CartId {
    async fn validate_entity(&self) -> AppResult<()> {
        Ok(())
    }
}

// * Name
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct CartName(String);

impl Deref for CartName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<CartName> for String {
    fn from(name: CartName) -> Self {
        name.0
    }
}

impl From<String> for CartName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[async_trait]
impl DataValidator for CartName {
    async fn validate_data(&self) -> AppResult<()> {
        Ok(())
    }
}

#[async_trait]
impl DomainEntityValidator for CartName {
    async fn validate_entity(&self) -> AppResult<()> {
        Ok(())
    }
}
