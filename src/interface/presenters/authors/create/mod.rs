use crate::interface::presenters::Endpoint;

mod model;
mod presenter;
mod view;

pub(super) struct Create;
impl Endpoint for Create {}
