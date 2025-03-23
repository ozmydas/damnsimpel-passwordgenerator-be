use axum::{routing::get, Router};

use super::handler;

pub fn hello_routes() -> Router {
    Router::new()
        .route("/", get(super::handler::hello_world))
        .route("/lorem", get(super::handler::lorem_ipsum))
        .route("/hello", get(super::handler::handler_hello))
        .route("/hello/{name}", get(super::handler::handler_hello2))
}
