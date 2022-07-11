use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    interface::services::{FieldSealed, StructInteractor},
    AppResult,
};

use super::fields::{PublisherId, PublisherName};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PublisherToBeValidated {
    id: FieldSealed<PublisherId>,
    name: FieldSealed<PublisherName>,
}

impl From<(FieldSealed<PublisherId>, FieldSealed<PublisherName>)> for PublisherToBeValidated {
    fn from(data: (FieldSealed<PublisherId>, FieldSealed<PublisherName>)) -> Self {
        Self {
            id: data.0,
            name: data.1,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Publisher {
    id: PublisherId,
    name: PublisherName,
}

impl From<Publisher> for (PublisherId, PublisherName) {
    fn from(value: Publisher) -> Self {
        let Publisher { id, name } = value;
        (id, name)
    }
}

impl From<(PublisherId, PublisherName)> for Publisher {
    fn from((id, name): (PublisherId, PublisherName)) -> Self {
        Self { id, name }
    }
}

#[async_trait]
impl StructInteractor<Publisher> for PublisherToBeValidated {
    async fn interact_struct(self) -> AppResult<Publisher> {
        Ok(Publisher {
            id: self.id.extract_field().await?,
            name: self.name.extract_field().await?,
        })
    }
}
