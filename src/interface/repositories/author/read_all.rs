use super::AuthorRepo;
use crate::{
    domain::entities::author::{AuthorId, AuthorName},
    usecases::FieldInteractor,
    AppResult,
};

use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Serialize)]
pub(crate) struct Author {
    id: AuthorId,
    name: AuthorName,
}

#[derive(Debug, FromRow)]
struct AuthorFromSQLx {
    id: i64,
    name: String,
}

#[derive(Debug, Serialize)]
struct AuthorInteractor {
    id: FieldInteractor<AuthorId>,
    name: FieldInteractor<AuthorName>,
}

impl TryFrom<AuthorFromSQLx> for AuthorInteractor {
    type Error = anyhow::Error;

    fn try_from(data: AuthorFromSQLx) -> Result<Self, Self::Error> {
        let AuthorFromSQLx { id, name } = data;
        Ok(AuthorInteractor {
            id: FieldInteractor::from(AuthorId::try_from(id)?),
            name: FieldInteractor::from(AuthorName::from(name)),
        })
    }
}
impl AuthorInteractor {
    async fn interact(self) -> AppResult<Author> {
        let Self { id, name } = self;
        Ok(Author {
            id: id.interact().await?,
            name: name.interact().await?,
        })
    }
}

impl AuthorRepo {
    pub(crate) async fn read_all(db_conn_pool: &SqlitePool) -> AppResult<Vec<Author>> {
        let records: Vec<AuthorFromSQLx> = sqlx::query_as!(
            AuthorFromSQLx,
            r#"
            SELECT id, name
            FROM `authors`;
        "#,
        )
        .fetch_all(db_conn_pool)
        .await?;

        // * To improve performance -> https://github.com/launchbadge/sqlx/issues/117

        let authors: Vec<AuthorInteractor> = records
            .into_iter()
            .map(|record| {
                AuthorInteractor::try_from(record).map_err(|error| {
                    tracing::error!("{error:?} - (file: {}, line: {})", file!(), line!());
                    error
                })
            })
            .filter_map(|record: AppResult<AuthorInteractor>| record.ok())
            .collect();
        let validated_authors = validate_authors(authors).await?;
        tracing::debug!("Authors: {:?}", &validated_authors);
        Ok(validated_authors)
    }
}

async fn validate_authors(authors: Vec<AuthorInteractor>) -> AppResult<Vec<Author>> {
    let mut validated_authors: Vec<Author> = Vec::new();
    for author_interactor in authors.into_iter() {
        validated_authors.push(Author::from(author_interactor.interact().await?));
    }
    Ok(validated_authors)
}
