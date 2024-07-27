
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use tracing::error;

use crate::error::{Results, Error};
use crate::utils::hash_password;

#[derive(Debug, Deserialize)]
pub struct RegisterBody {
    username: String,
    password: String

}
#[derive(Debug, Serialize)]
pub struct RegisterResp {
    message: String,
    userid: String
}

pub async fn register(
    State(pool) : State<PgPool>,
    Json(payload): Json<RegisterBody>
) -> Results<(StatusCode, Json<RegisterResp>)> {

    if payload.password.len() < 8 {
        error!("Length of password less than 8 characters");
        return Err(Error::RegisterFail { message: "Length of password less than 8 characters".to_string() });
    }

    if payload.username.len() > 16 {
        error!("Length of username more than 16 characters");
        return Err(Error::RegisterFail { message: "Length of username more than 16 characters".to_string() })
    }

    match sqlx::query("select username from users where username = $1")
        .bind(&payload.username)
        .fetch_one(&pool)
        .await {
            Ok(x) => {
                let username_asis: String = x.get("username");

                if &payload.username == &username_asis {
                    return Err(Error::RegisterFail { message: "username has been created".to_string() })
                }
            },
            Err(_) => {}
        }

    let (password, salt) = hash_password(&payload.password).unwrap();
    let uuid = uuid::Uuid::new_v4();
    let response = match sqlx::query("insert into users (id, username, password, salt, created_dt) values($1, $2, $3, $4, now())")
        .bind(&uuid)
        .bind(&payload.username)
        .bind(&password)
        .bind(&salt)
        .execute(&pool)
        .await {
            Ok(_) => {
                (
                    StatusCode::CREATED,
                    Json(RegisterResp {
                        message: "User has created".to_string(),
                        userid: uuid.to_string()})
                )
            },
            Err(e) => {
                return Err(Error::RegisterFail { message: e.to_string() })
            }
        };
    Ok(response)
}