#![allow(unused)]

use std::net::SocketAddr;

use axum::{middleware, response::Html, Router};
use middlewares::{authenticator::verify_request, response::main_response_mapper};
use web::{auth, hello};

mod models;
mod routes;
mod utils;
mod middlewares;
mod web;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes::app_routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn(verify_request))
        .fallback_service(routes::routes_static());

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();

    // start server
    let port = 4000;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!("--> 🔥 SERVER START on {port}\n");
    axum::serve(listener, app).await.unwrap();
    // end server
}
