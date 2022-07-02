use async_trait::async_trait;
use axum::{http::{StatusCode, HeaderMap}, response::IntoResponse, Extension, Json};
use serde::Serialize;
use sqlx::SqlitePool;


pub(crate) const X_TOTAL_COUNT: &'static str = "X-Total-Count";

#[async_trait]
pub(crate) trait Endpoint<ENTITY: Serialize> {
    async fn read_all(
        db_driver: Extension<SqlitePool>,
    ) -> Result<Json<Vec<ENTITY>>, (StatusCode, String)>;
    async fn count(
        db_driver: Extension<SqlitePool>,
    ) -> Result<(HeaderMap, ()), (StatusCode, String)>;
    // async fn read_all(db_driver: DBDRIVER, data_from_outside: DATAREQUEST) -> RESULT;
    // async fn read_one(db_driver: DBDRIVER, data_from_outside: DATAREQUEST) -> RESULT;
    // async fn count(db_driver: DBDRIVER, data_from_outside: DATAREQUEST) -> RESULT;
    // async fn count(db_driver: DBDRIVER, data_from_outside: DATAREQUEST) -> RESULT;
    // async fn count(db_driver: DBDRIVER, data_from_outside: DATAREQUEST) -> RESULT;
    // async fn count(db_driver: DBDRIVER, data_from_outside: DATAREQUEST) -> RESULT;

    // fn read_all<DBDRIVER, DATAREQUEST, DATARESPONSE>(
    //     db_driver: DBDRIVER,
    //     data_from_outside: DATAREQUEST,
    // ) -> Result<DATARESPONSE, (StatusCode, String)>
    // where
    //     DATAREQUEST: Into<String>,
    //     DATARESPONSE: IntoResponse;
    // fn read_one<DBDRIVER, DATAREQUEST, DATARESPONSE>(
    //     db_driver: DBDRIVER,
    //     data_from_outside: DATAREQUEST,
    // ) -> Result<DATARESPONSE, (StatusCode, String)>
    // where
    //     DATAREQUEST: Into<String>,
    //     DATARESPONSE: IntoResponse;
    // fn update<DBDRIVER, DATAREQUEST, DATARESPONSE>(
    //     db_driver: DBDRIVER,
    //     data_from_outside: DATAREQUEST,
    // ) -> Result<DATARESPONSE, (StatusCode, String)>
    // where
    //     DATAREQUEST: Into<String>,
    //     DATARESPONSE: IntoResponse;
    // fn delete<DBDRIVER, DATAREQUEST, DATARESPONSE>(
    //     db_driver: DBDRIVER,
    //     data_from_outside: DATAREQUEST,
    // ) -> Result<DATARESPONSE, (StatusCode, String)>
    // where
    //     DATAREQUEST: Into<String>,
    //     DATARESPONSE: IntoResponse;
}
