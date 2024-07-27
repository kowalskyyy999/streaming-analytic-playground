use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use rdkafka::producer::FutureRecord;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;

use rdkafka::{config::ClientConfig, producer::FutureProducer};

use crate::error::{Results, Error};
use crate::utils::kafka_producer_connection;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ProductsBody {
    userId: String,
    productId: i64,
    productName: String,
    timestamp: String
}

#[derive(Debug, Serialize)]
pub struct ProductsResp {
    message: String
}

pub async fn products(
    Json(payload): Json<ProductsBody>
) -> Results<(StatusCode, Json<ProductsResp>)> {
    let brokers = "kafka-server:9092";
    let topic_name = "channel1-topic";

    let kafka_payload = json!({
        "user_id": &payload.userId,
        "product_id": &payload.productId,
        "product_name": &payload.productName,
        "clicks_ts": &payload.timestamp
    });

    let kafka_producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "200")
        .create()
        .expect("Producer creation error");
    let kafka_producer = kafka_producer.clone();
    tokio::spawn(async move {
        let _ = kafka_producer.send(
            FutureRecord::to(&topic_name)
                .payload(&kafka_payload.to_string())
                .key(&payload.userId), 
            std::time::Duration::from_millis(200)).await;
    });

    let response = (
        StatusCode::CREATED,
        Json(ProductsResp { message: "Good".to_string()})
    );
    Ok(response)
}