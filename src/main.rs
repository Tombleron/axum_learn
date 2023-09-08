use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(|| async { Html("Hello, world!") }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("LISTENING ON {addr}");

    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}
