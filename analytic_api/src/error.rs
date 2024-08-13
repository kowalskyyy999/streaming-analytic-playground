use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub type Results<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TargetNotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let response = match self {
            Error::TargetNotFound => (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Target not found"})),
            ),
        };

        response.into_response()
    }
}
