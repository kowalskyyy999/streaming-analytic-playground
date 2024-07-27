use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub type Results<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    RegisterFail { message : String},
    SignUpErr { message: String},
    SignInErr { message: String}
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let response = match self {
            Error::RegisterFail { message } => {
                (StatusCode::BAD_REQUEST, Json(json!({"message": message})))
            },
            Error::SignUpErr { message } => {
                (StatusCode::BAD_REQUEST, Json(json!({"message": message})))
            },
            Error::SignInErr { message } => {
                (StatusCode::BAD_REQUEST, Json(json!({"message": message})))
            }
        };

        response.into_response()
    }
}