use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::{
    error::{Error, Results},
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct SigninBody {
    username: String,
    password: String,
}
#[derive(Debug, Serialize)]
pub struct SigninResp {
    message: String,
    userid: String,
}

impl Default for SigninResp {
    fn default() -> Self {
        SigninResp {
            message: String::new(),
            userid: String::new(),
        }
    }
}

// pub async fn signin(
//     State(pool) : State<PgPool>,
//     Json(payload): Json<SigninBody>
// ) -> Results<(StatusCode, Json<SigninResp>)> {

//     let response = match sqlx::query("select * from users where username = $1")
//         .bind(&payload.username)
//         .fetch_one(&pool)
//         .await {
//             Ok(x) => {
//                 let salt: String = x.get("salt");
//                 let ps = format!("{}{}", payload.password, salt);
//                 let digest = md5::compute(ps);
//                 let hash_s = format!("{:x}", digest);
//                 let hash_r: String = x.get("password");

//                 if hash_s == hash_r {
//                     let uuid: Uuid = x.get("id");
//                     (
//                         StatusCode::OK,
//                         Json(SigninResp { message: "username valid".to_string(), userid: uuid.to_string()})
//                     )
//                 } else {
//                     return Err(Error::SignInErr { message: "credential invalid".to_string() })
//                 }
//             },
//             Err(e) => {
//                 return Err(Error::SignInErr { message: e.to_string() })
//             }
//         };

//     Ok(response)
// }

pub async fn signin(
    State(state): State<AppState>,
    Json(payload): Json<SigninBody>,
) -> Results<(StatusCode, Json<SigninResp>)> {
    let pool = state.storage.lock().unwrap().read().unwrap().pool.clone();
    let mut response = (StatusCode::OK, Json(SigninResp::default()));

    {
        response = match sqlx::query("select * from users where username = $1")
            .bind(&payload.username)
            .fetch_one(&pool)
            .await
        {
            Ok(x) => {
                let salt: String = x.get("salt");
                let ps = format!("{}{}", payload.password, salt);
                let digest = md5::compute(ps);
                let hash_s = format!("{:x}", digest);
                let hash_r: String = x.get("password");

                if hash_s == hash_r {
                    let uuid: String = x.get("id");
                    (
                        StatusCode::OK,
                        Json(SigninResp {
                            message: "username valid".to_string(),
                            userid: uuid.to_string(),
                        }),
                    )
                } else {
                    return Err(Error::SignInErr {
                        message: "credential invalid".to_string(),
                    });
                }
            }
            Err(e) => {
                return Err(Error::SignInErr {
                    message: e.to_string(),
                })
            }
        };
    }

    Ok(response)
}
