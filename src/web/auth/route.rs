use axum::{routing::post, Router};

use super::handler::api_login;

pub fn account_routes() -> Router{
    Router::new()
        .route("/api/login", post(api_login))
}
