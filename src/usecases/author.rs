use async_trait::async_trait;
use serde::Deserialize;

use crate::{
    domain::entities::author::{AuthorId, AuthorName},
    interface::services::FieldSealed,
    AppResult,
};

use super::UseCaseValidator;

// * Id
#[async_trait]
impl UseCaseValidator for AuthorId {
    async fn validate_usecase(&self) -> AppResult<()> {
        // TODO
        Ok(())
    }
}

// * Name
#[async_trait]
impl UseCaseValidator for AuthorName {
    async fn validate_usecase(&self) -> AppResult<()> {
        // TODO
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct NewAuthor {
    pub(crate) name: FieldSealed<AuthorName>,
}
