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

pub(crate) fn get_routes_authors() -> Router {
    use crate::interface::presenters::authors::AuthorPresenter;
    use axum::routing::get;
    Router::new().route(
        "/authors",
        get(AuthorPresenter::read_all)
            .head(AuthorPresenter::count)
            .post(AuthorPresenter::create),
    )
}

pub(crate) fn get_routes_publishers() -> Router {
    use crate::interface::presenters::publishers::PublisherPresenter;
    use axum::routing::get;
    Router::new().route(
        "/publishers",
        get(PublisherPresenter::read_all)
            .head(PublisherPresenter::count)
            .post(PublisherPresenter::create),
    )
}
