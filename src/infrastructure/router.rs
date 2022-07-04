use axum::Router;

pub(crate) fn get_routes_books() -> Router {
    use crate::interface::presenters::books::BookPresenter;
    use axum::routing::get;
    Router::new().route(
        "/books",
        get(BookPresenter::read_all).head(BookPresenter::count), // .post(BookPresenter::create)
    )
    // .route(
    //     "/books/:id",
    //     get(BookPresenter::read_one)
    //         .patch(BookPresenter::update)
    //         .delete(BookPresenter::delete),
    // )
}
