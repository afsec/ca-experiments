use super::AuthorRepo;
use crate::{domain::entities::author::AuthorId, usecases::author::NewAuthor, AppResult};

use sqlx::SqlitePool;

impl AuthorRepo {
    pub(crate) async fn create(
        db_conn_pool: &SqlitePool,
        new_author: NewAuthor,
    ) -> AppResult<AuthorId> {
        let name = new_author.name.extract_field().await?;
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
