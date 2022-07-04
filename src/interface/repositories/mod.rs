// use async_trait::async_trait;
// use sqlx::{Database, Pool};

// use crate::AppResult;

pub(crate) mod book;
pub(crate) mod author;

// Repository – an object that provides access to persistent entities and encapsulates the
//  mechanism for accessing the database.

// #[async_trait]
// pub(crate) trait Repository<'endpoint, DBDRIVER, INPUTDATA, RETRIEVEDDATA>
// where
//     DBDRIVER: Database,
//     INPUTDATA: Send + Sync,
// {
//     async fn run(
//         &'endpoint self,
//         db_conn_pool: &Pool<DBDRIVER>,
//         input: INPUTDATA,
//     ) -> AppResult<RETRIEVEDDATA>
//     where
//         INPUTDATA: 'async_trait,
//         Self: Sized;
// }
