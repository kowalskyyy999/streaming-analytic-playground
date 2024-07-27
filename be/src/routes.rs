use axum::{routing::post, Router};
use crate::handlers::authentication::signup_handler::signup;
use crate::handlers::authentication::signin_handler::signin;
use crate::handlers::boards::boards_handler::boards;
use crate::handlers::boards::boards_reset_handler::board_reset;
use crate::handlers::boards::boards_submit_handler::board_submit;

use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/auth/signin", post(signin))
        .route("/auth/signup", post(signup))
        .route("/playground/boards", post(boards))
        .route("/playground/boards/reset", post(board_reset))
        .route("/playground/boards/submit", post(board_submit))
        .with_state(state)
}