use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    domain::entities::author::{AuthorId, AuthorName},
    AppResult,
};

use super::{UseCase, FieldInteractor};

// * Id

#[async_trait]
impl UseCase for AuthorId {
    async fn validate_usecase(&self) -> AppResult<()> {
        Ok(())
    }
}

// * Name

#[async_trait]
impl UseCase for AuthorName {
    async fn validate_usecase(&self) -> AppResult<()> {
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct NewAuthor {
    pub(crate) name: FieldInteractor<AuthorName>,
}