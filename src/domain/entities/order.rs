use std::ops::Deref;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{domain::DataValidator, AppResult};

use super::DomainEntityValidator;

// * Id
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct OrderId(u32);

impl Deref for OrderId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: Implement Validator

impl From<OrderId> for u32 {
    fn from(id: OrderId) -> Self {
        id.0
    }
}

impl From<u32> for OrderId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl TryFrom<i64> for OrderId {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}

impl TryFrom<OrderId> for i64 {
    type Error = anyhow::Error;

    fn try_from(data: OrderId) -> Result<Self, Self::Error> {
        Ok(Self::from(data.0))
    }
}

#[async_trait]
impl DataValidator for OrderId {
    async fn validate_data(&self) -> AppResult<()> {
        Ok(())
    }
}

#[async_trait]
impl DomainEntityValidator for OrderId {
    async fn validate_entity(&self) -> AppResult<()> {
        Ok(())
    }
}

// * Name
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct OrderName(String);

impl Deref for OrderName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<OrderName> for String {
    fn from(name: OrderName) -> Self {
        name.0
    }
}

impl From<String> for OrderName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[async_trait]
impl DataValidator for OrderName {
    async fn validate_data(&self) -> AppResult<()> {
        Ok(())
    }
}

#[async_trait]
impl DomainEntityValidator for OrderName {
    async fn validate_entity(&self) -> AppResult<()> {
        Ok(())
    }
}
