use std::{ops::Deref, sync::RwLock};
use axum::{
        extract::State, 
        http::StatusCode, 
        Json};
use serde::{Deserialize, Serialize};
use crate::error::Results;
use crate::state::{AppState, Buffer};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct BoardBody {
    userId: String,
    value: i32
}

#[derive(Debug, Serialize)]
pub struct BoardResp {
    message: String
}

pub async fn boards(
    State(state): State<AppState>,
    Json(payload): Json<BoardBody>
) -> Results<(StatusCode, Json<BoardResp>)> {
    let user_id = &payload.userId;
    let cache = state.storage.lock().unwrap().write().unwrap().cache.clone();
    let mut cache = cache.lock().unwrap();

    let lock_default = RwLock::new(Buffer{start_time: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true), num: 0});
    let mut buffer = Buffer::default();
    {
        let lock = cache.get(user_id).unwrap_or(&lock_default);
        let mut buffer_write = lock.write().unwrap();
        buffer_write.num += payload.value;

        buffer = buffer_write.deref().to_owned();
        println!("{:?}", buffer_write);
    }

    {
        cache.insert(user_id.to_string(), RwLock::new(buffer));
    }

    let response = (
        StatusCode::CREATED,
        Json(BoardResp { message: "Good".to_string()})
    );
    Ok(response)
}