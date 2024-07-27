
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use tracing::error;

use crate::error::{Results, Error};
use crate::state::AppState;
use crate::utils::hash_password;

#[derive(Debug, Deserialize)]
pub struct SignupBody {
    username: String,
    password: String

}
#[derive(Debug, Serialize)]
pub struct SignupResp {
    message: String,
    userid: String
}

impl Default for SignupResp {
    fn default() -> Self {
        SignupResp { message: String::new(), userid: String::new() }
    }
}

pub async fn signup(
    State(state) : State<AppState>,
    Json(payload): Json<SignupBody>
) -> Results<(StatusCode, Json<SignupResp>)> {
    let pool = state.storage.lock().unwrap().read().unwrap().pool.clone();
    // let mut response = (StatusCode::OK, Json(SignupResp::default()));

    if payload.password.len() < 8 {
        error!("Length of password less than 8 characters");
        return Err(Error::SignUpErr { message: "Length of password less than 8 characters".to_string() });
    }

    if payload.username.len() > 16 {
        error!("Length of username more than 16 characters");
        return Err(Error::SignUpErr { message: "Length of username more than 16 characters".to_string() })
    }

    match sqlx::query("select username from users where username = $1")
        .bind(&payload.username)
        .fetch_one(&pool)
        .await {
            Ok(x) => {
                let username_asis: String = x.get("username");

                if &payload.username == &username_asis {
                    return Err(Error::SignUpErr { message: "username has already created".to_string() })
                }
            },
            Err(_) => {}
        }

    let (password, salt) = hash_password(&payload.password).unwrap();
    let uuid = uuid::Uuid::new_v4();
    let response = match sqlx::query("insert into users (id, username, password, salt, created_dt) values($1, $2, $3, $4, now())")
        .bind(&uuid.to_string())
        .bind(&payload.username)
        .bind(&password)
        .bind(&salt)
        .execute(&pool)
        .await {
            Ok(_) => {
                (
                    StatusCode::CREATED,
                    Json(SignupResp {
                        message: "User has created".to_string(),
                        userid: uuid.to_string()})
                )
            },
            Err(e) => {
                return Err(Error::SignUpErr { message: e.to_string() })
            }
        };
    Ok(response)
}