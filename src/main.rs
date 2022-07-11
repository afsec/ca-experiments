// #![warn(clippy::all)]
#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::mem_forget,
    clippy::unused_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::await_holding_lock,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::fn_params_excessive_bools,
    clippy::exit,
    clippy::inefficient_to_string,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::verbose_file_reads,
    clippy::unnested_or_patterns,
    clippy::str_to_string,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    // missing_docs
    // missing_debug_implementations,
)]
#![deny(unreachable_pub, private_in_public)]
#![allow(elided_lifetimes_in_paths, clippy::type_complexity)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]

// Clean architecture - Reference: https://github.com/vidu171/clean-architecture-go
mod domain;
mod error;
mod infrastructure;
mod interface;
mod usecases;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use axum::{Extension, Router};
    use std::net::SocketAddr;
    use tower::ServiceBuilder;
    use tower_http::trace::TraceLayer;

    use crate::infrastructure::router;
    use infrastructure::sqlite::create_sqlite_pool;

    tracing_subscriber::fmt::init();
    // use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::EnvFilter::new(
    //         std::env::var("RUST_LOG").unwrap_or_else(|_| {
    //             // TODO: Replace to book defined log level
    //             format!("{}=info,tower_http=debug", env!("CARGO_PKG_NAME")).into()
    //         }),
    //     ))
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();
    let startup_message = "My Web service";
    tracing::info!("{}", startup_message);
    tracing::info!("Hello, world!");

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    // let sqlite_pool = create_sqlite_pool("./database.sqlite3").await?;

    // * Infrastructure (DB)
    let sqlite_pool = create_sqlite_pool("./database.sqlite3").await?;

    // * Infrastructure (Router)
    let routes = Router::new()
        .merge(router::get_routes_authors())
        .merge(router::get_routes_books())
        .merge(router::get_routes_publishers())
        .merge(router::get_routes_orders())
        .merge(router::get_routes_carts());

    // * Infrastructure (Router)
    let app = Router::new()
        .nest("/api", routes)
        .layer(ServiceBuilder::new().layer(Extension(sqlite_pool)))
        .layer(TraceLayer::new_for_http());

    tracing::info!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

type AppResult<T> = anyhow::Result<T>;
