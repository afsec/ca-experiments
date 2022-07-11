use async_trait::async_trait;
use sqlx::Sqlite;

use crate::{
    domain::entities::publisher::structs::{Publisher, PublisherToBeValidated},
    // interface::repositories::publisher::read_all::RepoPublisherReadAll,
    AppResult,
};

use super::{Service, StructInteractor};

pub(crate) struct ServicePublisherReadAll;

#[async_trait]
impl StructInteractor<Vec<Publisher>> for Vec<PublisherToBeValidated> {
    async fn interact_struct(self) -> AppResult<Vec<Publisher>> {
        self.iter().map(|item| item);
        // TODO:
        Ok(vec![])
    }
}
#[async_trait]
impl StructInteractor<()> for () {
    async fn interact_struct(self) -> AppResult<()> {
        Ok(())
    }
}

// TODO:
// #[async_trait]
// impl<'endpoint>
//     Service<
//         Sqlite,
//         RepoPublisherReadAll,
//         (),
//         (),
//         Vec<Publisher>,
//         Vec<PublisherValidated>,
//     > for ServicePublisherReadAll
// {
// }
