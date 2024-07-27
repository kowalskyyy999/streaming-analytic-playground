use std::ops::Deref;
use std::str::FromStr;

use axum::Extension;
use axum::{
        extract::State, 
        http::StatusCode,
        Json};
use chrono::{NaiveDate, NaiveDateTime};
use rdkafka::producer::FutureRecord;
use serde::{Deserialize, Serialize};

use rdkafka::{config::ClientConfig, producer::FutureProducer};
use serde_json::json;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::error::Results;
use crate::state::AppState;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct SubmitBody {
    userId: String,
    target: i32,
    clicks: i32
}

#[derive(Debug, Serialize)]
pub struct SubmitResp {
    message: String,
    refresh: i32
}

pub async fn board_submit(
    State(state): State<AppState>,
    Json(payload): Json<SubmitBody>
) -> Results<(StatusCode, Json<SubmitResp>)> {

    let user_id = &payload.userId;
    let user_id_cln = user_id.clone();

    let target = payload.target;
    let mut start_time_str = String::new();
    let mut finish_time_str = String::new();

    let pool = state.storage.lock().unwrap().read().unwrap().pool.clone();
    let cache = state.storage.lock().unwrap().write().unwrap().cache.clone();
    let mut cache = cache.lock().unwrap();

    let mut is_match: bool;
    {
        let lock = cache.get(user_id).unwrap();
        let buffer = lock.read().unwrap();
        
        is_match = buffer.num == target;
        println!("Is Match => {}", is_match);

        if is_match {
            let start_time = &buffer.start_time;
            let finish_time = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

            // let brokers = "kafka-server:9092";
            let brokers = "kafka:9092";
            let topic_name = "channel1-topic";

            let start_time_kafka = NaiveDateTime::parse_from_str(&start_time, "%Y-%m-%dT%H:%M:%SZ").unwrap();
            let finish_time_kafka = NaiveDateTime::parse_from_str(&finish_time, "%Y-%m-%dT%H:%M:%SZ").unwrap();
            
            let kafka_payload = json!({
                "user_id": &user_id_cln,
                "start_time": &start_time_kafka.to_string(),
                "finish_time": &finish_time_kafka.to_string(),
                "target": target,
                "count_clicks": payload.clicks
            });

            // start_time_str = start_time_kafka;
            // finish_time_str = finish_time_kafka;

            let uuid_user = Uuid::from_str(&user_id.clone().as_str()).unwrap();

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
                        .key(&user_id_cln), 
                        std::time::Duration::from_millis(200)).await;


            });

            tokio::spawn(async move {
                let _ = sqlx::query("insert into history_champs (id, start_time, finish_time, target, count_click) values ($1, $2, $3, $4, $5)")
                        .bind(&uuid_user.to_string())
                        .bind(&start_time_kafka)
                        .bind(&finish_time_kafka)
                        .bind(target)
                        .bind(payload.clicks)
                        .execute(&pool)
                        .await
                        .unwrap();
            });

            // tokio::select! {}
        }
    }

    if is_match {
        cache.remove(user_id).unwrap();
        return Ok(
            (
                StatusCode::OK,
                Json(SubmitResp { message: "Congratulations, your calculation is match".to_string(), refresh: 1 })
            )
        )
    }
    let response = (
        StatusCode::NOT_FOUND,
        Json(SubmitResp { message: "Sorry, your calculation is wrong".to_string(), refresh: 0})
    );

    Ok(response)
}