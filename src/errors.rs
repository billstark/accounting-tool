use axum::{response::IntoResponse, http::StatusCode};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InternalServerError,
    BadRequest
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_REQUEST").into_response()
            },
            Error::BadRequest => {
                (StatusCode::BAD_REQUEST, "BAD_REQUEST").into_response()
            }
        }
    }
}

