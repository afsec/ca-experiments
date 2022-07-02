use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct User {
    id: u16,
    name: String,
}
