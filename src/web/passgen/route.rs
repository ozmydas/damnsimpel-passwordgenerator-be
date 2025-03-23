use axum::{routing::post, Router};

use super::handler::api_passgen;

pub fn passgen_routes() -> Router{
    Router::new()
        .route("/api/passgen", post(api_passgen))
}
