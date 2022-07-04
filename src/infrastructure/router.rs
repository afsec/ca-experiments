use axum::Router;

pub(crate) fn get_routes_users() -> Router {
    use crate::helpers::crud::Crud;
    use crate::interface::presenters::users::EndpointUsers;
    use axum::routing::get;
    Router::new().route(
        "/users",
        get(EndpointUsers::read_all).head(EndpointUsers::count), // .post(EndpointUsers::create)
    )
    // .route(
    //     "/users/:id",
    //     get(EndpointUsers::read_one)
    //         .patch(EndpointUsers::update)
    //         .delete(EndpointUsers::delete),
    // )
}
