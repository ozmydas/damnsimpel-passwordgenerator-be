use crate::{web::{auth, hello, passgen}};
use axum::{
    routing::{get, get_service, post},
    Router,
};
use tower_http::services::ServeDir;

pub fn app_routes() -> Router {
    Router::new()
        .merge(hello::route::hello_routes())
        .merge(auth::route::account_routes())
        .merge(passgen::route::passgen_routes())
}

pub fn routes_static() -> Router {
    Router::new().nest_service("/files", get_service(ServeDir::new("./files")))
}
