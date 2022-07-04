use axum::Router;

pub(crate) fn get_routes_books() -> Router {
    use crate::interface::crud::Crud;
    use crate::interface::presenters::books::EndpointBooks;
    use axum::routing::get;
    Router::new().route(
        "/books",
        get(EndpointBooks::read_all).head(EndpointBooks::count), // .post(EndpointBooks::create)
    )
    // .route(
    //     "/books/:id",
    //     get(EndpointBooks::read_one)
    //         .patch(EndpointBooks::update)
    //         .delete(EndpointBooks::delete),
    // )
}
