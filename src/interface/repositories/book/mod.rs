use sqlx::{Database, Pool, Sqlite};

use super::Repository;

pub(crate) struct BookRepo<DBDRIVER: Database>(Pool<DBDRIVER>);

impl Repository for BookRepo<Sqlite> {}
