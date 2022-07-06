use std::{fmt::Display, num::TryFromIntError};

use thiserror::Error;

#[derive(Error, Debug, Default, PartialEq, Eq)]
pub(crate) enum AppError {
    #[default]
    Nil,
    TryFromIntError(#[from] TryFromIntError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message: String = match &*self {
            AppError::TryFromIntError(_) => "Datastore invalid data".into(),
            _ => "IMPOSSIBLE STATE!".into(),
        };
        write!(f, "{message}")
    }
}
