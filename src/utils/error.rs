use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Loginfail,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - Error Response - {self:?}", "INTO RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
