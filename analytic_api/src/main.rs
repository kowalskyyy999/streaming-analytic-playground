mod error;
mod handler;
mod handlers;
mod pool;

use axum::http::{header::CONTENT_TYPE, Method};
use axum::routing::get;
use axum::Router;
use r2d2_mysql::{
    mysql::{prelude::*, Opts, OptsBuilder},
    r2d2, MySqlConnectionManager,
};
use tower_http::{
    cors::Any,
    trace::{self, TraceLayer},
};
use tracing::{info, Level};

use serde::{Deserialize, Serialize};

use crate::handler::*;
use crate::pool::{AppState, StarrocksConnection};

#[derive(Debug, Serialize)]
struct ResultData {
    target: String,
    username: String,
    clicks: i32,
    ranking: i32,
}

#[tokio::main]
async fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let state = StarrocksConnection::new(
        String::from("root"),
        None,
        String::from("0.0.0.0"),
        9038,
        String::from("stream_db"),
    )
    .unwrap()
    .connect()
    .unwrap();

    let app = Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .route("/fastest/:target", get(fastest_handler))
                .route("/shortest/:target", get(shortest_handler))
                .route("/fastest/:target/today", get(fastest_today_handler))
                .route("/shortest/:target/today", get(shortest_today_handler))
                .with_state(state),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(Any)
                .allow_headers([CONTENT_TYPE])
                .allow_methods([Method::GET]),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8765").await.unwrap();
    info!("listening on {:?}", listener);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
