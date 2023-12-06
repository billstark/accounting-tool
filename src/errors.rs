use axum::{response::IntoResponse, http::StatusCode};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InternalServerError
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_REQUEST").into_response()
    }
}

