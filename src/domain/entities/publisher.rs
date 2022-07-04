use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct PublisherId(u32);

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct PublisherName(String);

// #[derive(Debug, Deserialize, Serialize)]
// pub(crate) struct Publisher {
//     id: PublisherId,
//     name: PublisherName,
// }
