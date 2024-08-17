use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex, RwLock},
};

use axum::http::{header::CONTENT_TYPE, Method};
use dotenv;
use sqlx::postgres::PgPoolOptions;
use state::Storage;
use tower_http::{
    cors::Any,
    trace::{self, TraceLayer},
};
use tracing::{info, Level};

use crate::routes::router;
use crate::state::AppState;

mod error;
mod handlers;
mod routes;
mod state;
mod utils;

#[tokio::main]
async fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    if let Ok(_) = std::fs::File::open(".env") {
        dotenv::dotenv().ok();
    };

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let pg_user = env::var("POSTGRES_USER").expect("please fill the database user");
    let pg_pass = env::var("POSTGRES_PASSWORD").expect("please fill the database password");
    let pg_host = env::var("POSTGRES_HOST").expect("please fill the database host");
    let pg_port = env::var("POSTGRES_PORT").expect("please fill the database port");
    let pg_db = env::var("POSTGRES_DB").expect("please fill the database name");

    let url = format!("postgres://{pg_user}:{pg_pass}@{pg_host}:{pg_port}/{pg_db}");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&url)
        .await
        .expect("Failed connect Postgres database server");

    {
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    }

    let storage = Storage {
        pool: pool,
        cache: Arc::new(Mutex::new(HashMap::default())),
    };
    let state = AppState {
        storage: Arc::new(Mutex::new(RwLock::new(storage))),
    };

    let app = axum::Router::new()
        .nest("/api", router(state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(Any)
                // .allow_headers([CONTENT_TYPE])
                .allow_methods([Method::POST]),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9990").await.unwrap();
    info!("listening on {:?}", listener);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
