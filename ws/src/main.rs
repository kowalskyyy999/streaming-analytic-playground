use axum::extract::State;
use axum::extract::{
    connect_info::ConnectInfo,
    ws::{Message as MessageWs, WebSocket, WebSocketUpgrade},
};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

use dotenv;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::message::Message;
use rdkafka::ClientConfig;
use serde_json::json;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

use tower_http::trace::{self, TraceLayer};
use tracing::{error, info, Level};

#[derive(Debug, Clone, Default)]
struct InMemory {
    latest_ts: String,
    user_active: String,
}

impl std::fmt::Display for InMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#" {{"latest_ts": "{}", "user_active": "{}" }}"#,
            self.latest_ts, self.user_active
        )
    }
}

#[tokio::main]
async fn main() {
    if let Ok(_) = std::fs::File::open(".env") {
        dotenv::dotenv().ok();
    };

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app_state = Arc::new(RwLock::new(InMemory::default()));

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9991").await.unwrap();

    info!("listening websocket on: {:?}", listener);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(app_state): State<Arc<RwLock<InMemory>>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr, app_state))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr, app_state: Arc<RwLock<InMemory>>) {
    let brokers = env::var("KAFKA_BROKER").unwrap();
    let topic = env::var("KAFKA_TOPIC").unwrap();
    let topics = [topic.as_str()];

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "test")
        .set("bootstrap.servers", brokers)
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .create()
        .unwrap();

    consumer.subscribe(&topics).unwrap();

    let cache = app_state.clone();
    let mut writer = tokio::spawn(async move {
        loop {
            match consumer.recv().await {
                Err(error) => {
                    error!("{error}");
                    // println!("{error}")
                }
                Ok(data) => {
                    let payload = match data.payload_view::<str>() {
                        None => {
                            let now = chrono::Utc::now();
                            let ts = now.format("%Y-%m-%d %H:%M:%S").to_string();
                            json!({"ts": &ts, "uv": 0})
                        }
                        Some(x) => serde_json::from_str(&x.unwrap()).unwrap(),
                    };

                    let ts = payload.get("ts").unwrap().to_string();
                    let uv = payload.get("uv").unwrap();

                    let mut writer = cache.write().await;
                    let mem = InMemory {
                        latest_ts: ts.to_string().trim_matches('"').replace("\\\"", ""),
                        user_active: uv.to_string(),
                    };
                    *writer = mem;
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    let reader_cache = app_state.clone();
    let mut reader = tokio::spawn(async move {
        loop {
            let reader = reader_cache.read().await;
            if socket
                .send(MessageWs::Text(reader.to_string()))
                .await
                .is_err()
            {
                error!("Client disconnected");
                // println!("Client disconnected");
                return;
            }

            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    });

    let updater_cache = app_state.clone();
    let mut updater = tokio::spawn(async move {
        loop {
            let mut writer = updater_cache.write().await;

            let now_utc = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let now = chrono::NaiveDateTime::parse_from_str(now_utc.as_str(), "%Y-%m-%d %H:%M:%S")
                .unwrap();

            let latest_ts = &writer.latest_ts;
            let latest =
                chrono::NaiveDateTime::parse_from_str(latest_ts.as_str(), "%Y-%m-%d %H:%M:%S")
                    .unwrap_or(now);
            let elapse = now - latest;
            if elapse.num_seconds() > 0 {
                let mem = InMemory {
                    latest_ts: now_utc,
                    user_active: String::from("0"),
                };
                *writer = mem;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    });

    tokio::select! {
        _ = (&mut writer) => writer.abort(),
        _ = (&mut reader) => reader.abort(),
        _ = (&mut updater) => updater.abort(),
    }
}
