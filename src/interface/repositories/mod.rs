use async_trait::async_trait;
use serde::Serialize;
use sqlx::{Database, Pool};

use crate::AppResult;

pub(crate) mod author;
pub(crate) mod book;
pub(crate) mod cart;
pub(crate) mod order;
pub(crate) mod publisher;

#[async_trait]
pub(crate) trait Repository<DBDRIVER, INPUTDATA, OUTCOME>
where
    DBDRIVER: Database,
    OUTCOME: Serialize + Send,
    INPUTDATA: Send + Sync,
{
    async fn repository(
        db_conn_pool: &Pool<DBDRIVER>,
        submitted_data: INPUTDATA,
    ) -> AppResult<OUTCOME>
    where
        INPUTDATA: 'async_trait;
}

/*
pub(crate) trait Repository<
    'endpoint,
    DBDRIVER,
    INPUTDATA,
    VALIDATEDINPUTDATA,
    OUTCOME,
> where
    DBDRIVER: Database,
    INPUTDATA: Send + Sync + DeserializeOwned,
    VALIDATEDINPUTDATA: Send + Sync + StructInteractor<INPUTDATA>,
    OUTCOME: Send + Sync + DeserializeOwned + Serialize,
{
    async fn repository(
        &'endpoint self,
        db_conn_pool: &Pool<DBDRIVER>,
        submitted_data: VALIDATEDINPUTDATA,
    ) -> AppResult<OUTCOME>
    where
        VALIDATEDINPUTDATA: 'async_trait;
}

*/
