mod error;
mod handler;
mod job;
mod pool;

use axum::http::{header::CONTENT_TYPE, Method};
use axum::routing::get;
use axum::Router;
use dotenv;
use r2d2_mysql::{
    mysql::{prelude::*, Opts, OptsBuilder},
    r2d2, MySqlConnectionManager,
};
use serde::{Deserialize, Serialize};
use std::env;
use tower_http::{
    cors::Any,
    trace::{self, TraceLayer},
};
use tracing::{error, info, Level};

use crate::handler::*;
use crate::job::Job;
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
    if let Ok(_) = std::fs::File::open(".env") {
        dotenv::dotenv().ok();
    };

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let starrocks_user = env::var("STARROCKS_USER").unwrap();
    let starrocks_password = match env::var("STARROCKS_PASSWORD") {
        Ok(x) => {
            if x.is_empty() {
                None
            } else {
                Some(x)
            }
        }
        Err(_) => None,
    };
    let starrocks_host = env::var("STARROCKS_HOST").unwrap();
    let starrocks_port = env::var("STARROCKS_PORT").unwrap().parse::<u64>().unwrap();
    let starrocks_db = String::from("stream_db");

    let job_worker = Job::config(
        &starrocks_user,
        &starrocks_password,
        &starrocks_host,
        starrocks_port,
    );
    let _ = job_worker.initialize().unwrap();

    let state = StarrocksConnection::new(
        starrocks_user,
        starrocks_password,
        starrocks_host,
        starrocks_port,
        starrocks_db,
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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9992").await.unwrap();
    info!("listening on {:?}", listener);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
