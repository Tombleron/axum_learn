mod ctx;
mod error;
mod log;
mod model;
mod web;

use ctx::Ctx;
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use error::{Error, Result};

use axum::{
    extract::{Path, Query},
    http::{Method, Uri},
    middleware,
    response::Response,
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Json, Router,
};

use crate::{log::log_request, model::ModelControler};

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelControler::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::require_auth));

    let routes_hello = Router::new()
        .merge(web::routes_login::routes())
        .merge(routes_hello())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("LISTENING ON {addr}");

    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    let uuid = uuid::Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|e| e.client_status_and_error());

    let response_error = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            }
            );

            println!(
                "->> {:<12} - client_error_body: {client_error_body}",
                "RES_MAPPER"
            );

            (*status_code, Json(client_error_body)).into_response()
        });

    println!();

    let client_error = client_status_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    response_error.unwrap_or(res)
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
