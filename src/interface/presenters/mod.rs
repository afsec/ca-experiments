use crate::AppResult;
use async_trait::async_trait;
use axum::http::StatusCode;
use serde::Serialize;
use sqlx::{Database, Pool};

pub(crate) mod authors;
pub(crate) mod books;
pub(crate) mod carts;
pub(crate) mod orders;
pub(crate) mod publishers;

const X_TOTAL_COUNT: &'static str = "X-Total-Count";

//////////////////////////////////////////////////
trait Endpoint {}

#[async_trait]
trait Model<'endpoint, DBDRIVER, INPUTDATA, OUTCOME>
where
    DBDRIVER: Database,
    OUTCOME: Serialize + Send,
    INPUTDATA: Send + Sync,
{
    async fn model(
        &'endpoint self,
        db_conn_pool: &Pool<DBDRIVER>,
        submitted_data: INPUTDATA,
    ) -> AppResult<OUTCOME>
    where
        INPUTDATA: 'async_trait;
}

#[async_trait]
trait View<'endpoint, OUTCOME, INTORESPONSE>
where
    OUTCOME: Serialize + Send,
{
    async fn view(
        &'endpoint self,
        result: AppResult<OUTCOME>,
    ) -> Result<INTORESPONSE, (StatusCode, String)>;
}

#[async_trait]
trait Presenter<'endpoint, ENDPOINT, DBDRIVER, INPUTDATA, OUTCOME, INTORESPONSE>
where
    ENDPOINT: Endpoint
        // + Name
        + Model<'endpoint, DBDRIVER, INPUTDATA, OUTCOME>
        + View<'endpoint, OUTCOME, INTORESPONSE>
        + Send
        + Sync
        + 'endpoint,
    DBDRIVER: Database,
    INPUTDATA: Send + Sync,
    OUTCOME: Serialize + Send,
{
    async fn presenter(
        endpoint: &'endpoint ENDPOINT,
        db_conn_pool: &Pool<DBDRIVER>,
        submitted_data: INPUTDATA,
    ) -> Result<INTORESPONSE, (StatusCode, String)>
    where
        INPUTDATA: 'async_trait,
    {
        let model_result = endpoint.model(db_conn_pool, submitted_data).await;
        let view_result = endpoint.view(model_result).await;
        view_result
    }
}
