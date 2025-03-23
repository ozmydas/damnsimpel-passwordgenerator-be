use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
};

use crate::models;

pub async fn hello_world() -> impl IntoResponse {
    "Hello world!"
}

pub async fn lorem_ipsum() -> impl IntoResponse {
    "Lorem ipsum dolor sit amet"
}

// e.g. >> /hello?name=anu
pub async fn handler_hello(Query(params): Query<models::hello::HelloParam>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");

    Html(format!("<h1>Hello {}</h1>", name))
}

// e.g. >> /hello/{anu}
pub async  fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse{
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("<h1>Hello2 {}</h1>", name))
}
