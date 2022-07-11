use crate::{
    domain::{entities::DomainEntityValidator, DataValidator},
    usecases::UseCaseValidator,
    AppResult,
};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::{Database, Pool};

use super::repositories::Repository;

pub(crate) mod publisher;

/// Service – an object that implements business logic which doesn’t belong in an entity or
///  a value object.
// pub(crate) trait Service {}

#[async_trait]
pub(crate) trait Service<
    DBDRIVER,
    REPOSITORY,
    INPUTDATA,
    VALIDATEDINPUTDATA,
    OUTCOME,
    VALIDATEDOUTCOME,
> where
    DBDRIVER: Database,
    REPOSITORY: Send + Sync + Repository<DBDRIVER, VALIDATEDINPUTDATA, OUTCOME>,
    INPUTDATA: Send + Sync + DeserializeOwned + Serialize + StructInteractor<VALIDATEDINPUTDATA>,
    VALIDATEDINPUTDATA: Send + Sync + DeserializeOwned + Serialize,
    OUTCOME: Send + Sync + DeserializeOwned + Serialize + StructInteractor<VALIDATEDOUTCOME>,
    VALIDATEDOUTCOME: Send + Sync + DeserializeOwned + Serialize,
{
    async fn service(
        db_conn_pool: &Pool<DBDRIVER>,
        submitted_data: INPUTDATA,
    ) -> AppResult<VALIDATEDOUTCOME>
    where
        INPUTDATA: 'async_trait,
        REPOSITORY: 'async_trait,
    {
        let validated_input_data = submitted_data.interact_struct().await?;
        let outcome = REPOSITORY::repository(db_conn_pool, validated_input_data).await?;
        outcome.interact_struct().await
    }
}

////////////////////////////////////////////////////////

#[async_trait]
pub(crate) trait StructInteractor<T>
where
    Self: DeserializeOwned + Serialize,
    T: DeserializeOwned + Serialize,
{
    async fn interact_struct(self) -> AppResult<T>;
}

// #[derive(Debug, Deserialize, Serialize)]
// pub(crate) struct StructInteractor<T>(T)
// where
//     T: DataValidator + DomainEntityValidator + UseCaseValidator;

// impl<T> StructInteractor<T>
// where
//     T: DataValidator + DomainEntityValidator + UseCaseValidator,
// {
//     pub(crate) async fn interact_struct(self) -> AppResult<T> {
//         self.0.analyze_fields().await?;
//         Ok(self.0)
//     }
// }

// impl<T> From<T> for StructInteractor<T>
// where
//     T: DataValidator + DomainEntityValidator + UseCaseValidator,
// {
//     fn from(data: T) -> Self {
//         Self(data)
//     }
// }

////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct FieldInteractor<T>(T)
where
    T: DataValidator + DomainEntityValidator + UseCaseValidator;

impl<T> FieldInteractor<T>
where
    T: DataValidator + DomainEntityValidator + UseCaseValidator,
{
    pub(crate) async fn interact_field(self) -> AppResult<T> {
        self.0.validate_data().await?;
        self.0.validate_entity().await?;
        self.0.validate_usecase().await?;
        Ok(self.0)
    }
}

impl<T> From<T> for FieldInteractor<T>
where
    T: DataValidator + DomainEntityValidator + UseCaseValidator,
{
    fn from(data: T) -> Self {
        Self(data)
    }
}
