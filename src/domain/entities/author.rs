use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AuthorId(u32);

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AuthorName(String);

// #[derive(Debug, Deserialize, Serialize)]
// pub(crate) struct Author {
//     id: AuthorId,
//     name: AuthorName,
// }
