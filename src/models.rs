use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct User {
    id: UserId,
    name: UserName,
}

#[derive(Debug, Serialize)]
pub(crate) struct UserId(u16);

#[derive(Debug, Serialize)]
pub(crate) struct UserName(String);

