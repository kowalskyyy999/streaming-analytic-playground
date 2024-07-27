use axum::{
    extract::State, 
    http::StatusCode,
    Json};
use serde::{Deserialize, Serialize};
use crate::error::Results;
use crate::state::AppState;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ResetBody {
    userId: String
}

#[derive(Debug, Serialize)]
pub struct ResetResp {
    message: String
}

pub async fn board_reset(
        State(state): State<AppState>,
        Json(payload): Json<ResetBody>
    ) -> Results<(StatusCode, Json<ResetResp>)> {

    let user_id = &payload.userId;
    let cache = state.storage.lock().unwrap().write().unwrap().cache.clone();
    let mut cache = cache.lock().unwrap();

    {
        let lock = cache.get(user_id).unwrap();
        let mut buffer_write = lock.write().unwrap();
        buffer_write.num = 0;
    }
    
    let response = (
        StatusCode::OK,
        Json(ResetResp { message: "Successfully reset".to_string()})
    );
    Ok(response)
}