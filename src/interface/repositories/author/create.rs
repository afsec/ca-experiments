use super::AuthorRepo;
use crate::{
    domain::entities::author::{AuthorId, AuthorName},
    AppResult, usecases::Interactor,
};
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct NewAuthor {
    pub(crate) name: Interactor<AuthorName>,
}

impl AuthorRepo {
    pub(crate) async fn create(
        db_conn_pool: &SqlitePool,
        new_author: NewAuthor,
    ) -> AppResult<AuthorId> {
        let name = new_author.name.interact().await?;
        let rowid = sqlx::query!(
            r#"
                INSERT INTO `authors` ("name")
                VALUES (?1);
            "#,
            *name,
        )
        .execute(db_conn_pool)
        .await?
        .last_insert_rowid();
        Ok(rowid.try_into()?)
    }
}
