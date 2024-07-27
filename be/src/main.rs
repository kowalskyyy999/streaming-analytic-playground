use std::{collections::HashMap, sync::{Arc, Mutex, RwLock}};

use axum::http::{header::CONTENT_TYPE, Method};
use sqlx::postgres::PgPoolOptions;
use state::Storage;
use tower_http::{cors::Any, trace::{self, TraceLayer}};
use tracing::{Level, info};

use crate::routes::router;
use crate::state::AppState;

mod error;
mod routes;
mod handlers;
mod utils;
mod state;

#[tokio::main]
async fn main() -> core::result::Result<(), Box<dyn std::error::Error>>{
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let url = "postgres://kowalskyyy:Password123456@localhost:5433/kowlsss_db";

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(url)
        .await
        .expect("Failed connect Postgres database server");

    {
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    }

    let storage = Storage {
        pool: pool,
        cache: Arc::new(Mutex::new(HashMap::default()))
    };
    let state = AppState {
        storage: Arc::new(Mutex::new(RwLock::new(storage)))
    };

    let app = axum::Router::new()
        .nest("/api", router(state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new()
                    .level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new()
                    .level(Level::INFO)))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(Any)
                .allow_headers([CONTENT_TYPE])
                .allow_methods([Method::POST]));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9090").await.unwrap();
    info!("listening on {:?}", listener);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
