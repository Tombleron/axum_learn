mod error;
mod model;
mod web;

use serde::Deserialize;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use error::{Error, Result};

use axum::{
    extract::{Path, Query},
    middleware,
    response::Response,
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .merge(web::routes_login::routes())
        .merge(routes_hello())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("LISTENING ON {addr}");

    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
        .route("/hello/:name", get(hello_handler_path))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn hello_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello, {name}!"))
}

async fn hello_handler_path(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello_path", "HANDLER");

    Html(format!("Hello, {name}!"))
}
