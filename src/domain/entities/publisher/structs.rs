use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    interface::services::{FieldInteractor, StructInteractor},
    AppResult,
};

use super::fields::{PublisherId, PublisherName};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PublisherToBeValidated {
    id: FieldInteractor<PublisherId>,
    name: FieldInteractor<PublisherName>,
}

impl From<(FieldInteractor<PublisherId>, FieldInteractor<PublisherName>)>
    for PublisherToBeValidated
{
    fn from(data: (FieldInteractor<PublisherId>, FieldInteractor<PublisherName>)) -> Self {
        Self {
            id: data.0,
            name: data.1,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Publisher {
    // TODO:
    pub(crate) id: PublisherId,
    // TODO:
    pub(crate) name: PublisherName,
}

#[async_trait]
impl StructInteractor<Publisher> for PublisherToBeValidated {
    async fn interact_struct(self) -> AppResult<Publisher> {
        Ok(Publisher {
            id: self.id.interact_field().await?,
            name: self.name.interact_field().await?,
        })
    }
}
