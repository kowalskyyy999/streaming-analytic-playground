use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use r2d2_mysql::mysql::prelude::*;
use r2d2_mysql::r2d2::*;
use serde::Serialize;

use crate::error::{Error, Results};
use crate::pool::PoolState;

#[derive(Debug, Serialize)]
pub struct Responses {
    message: String,
}

#[derive(Debug, Serialize)]
pub struct OutputFastest {
    username: String,
    ranking: i32,
    times: i32,
}

#[derive(Debug, Serialize)]
pub struct OutputShortest {
    username: String,
    clicks: i32,
    ranking: i32,
}

pub async fn fastest_handler(
    Path(target): Path<i32>,
    State(state): State<PoolState>,
) -> Results<(StatusCode, Json<Vec<OutputFastest>>)> {
    let mut pool = state.clone().get().unwrap();
    let query = format!(
        "select username, times, ranking from mv_fastest where target = '{}' order by ranking",
        target
    );

    let rows = pool
        .query_map(query, |(username, times, ranking)| OutputFastest {
            username,
            ranking,
            times,
        })
        .unwrap();

    if rows.len() < 1 {
        return Err(Error::TargetNotFound);
    }

    let response = (StatusCode::OK, Json(rows));

    Ok(response)
}

pub async fn shortest_handler(
    Path(target): Path<i32>,
    State(state): State<PoolState>,
) -> Results<(StatusCode, Json<Vec<OutputShortest>>)> {
    let mut pool = state.clone().get().unwrap();
    let query = format!(
        "select username, clicks, ranking from mv_shortest where target = '{}' order by ranking",
        target
    );

    let rows = pool
        .query_map(query, |(username, clicks, ranking)| OutputShortest {
            username,
            clicks,
            ranking,
        })
        .unwrap();

    if rows.len() < 1 {
        return Err(Error::TargetNotFound);
    }

    let response = (StatusCode::OK, Json(rows));

    Ok(response)
}

pub async fn fastest_today_handler(
    Path(target): Path<i32>,
    State(state): State<PoolState>,
) -> Results<(StatusCode, Json<Vec<OutputFastest>>)> {
    let mut pool = state.clone().get().unwrap();
    let query = format!(
        "select username, times, ranking from mv_fastest_today where target = '{}' order by ranking",
        target
    );

    let rows = pool
        .query_map(query, |(username, times, ranking)| OutputFastest {
            username,
            ranking,
            times,
        })
        .unwrap();

    if rows.len() < 1 {
        return Err(Error::TargetNotFound);
    }

    let response = (StatusCode::OK, Json(rows));

    Ok(response)
}

pub async fn shortest_today_handler(
    Path(target): Path<i32>,
    State(state): State<PoolState>,
) -> Results<(StatusCode, Json<Vec<OutputShortest>>)> {
    let mut pool = state.clone().get().unwrap();
    let query = format!(
        "select username, clicks, ranking from mv_shortest_today where target = '{}' order by ranking",
        target
    );

    let rows = pool
        .query_map(query, |(username, clicks, ranking)| OutputShortest {
            username,
            clicks,
            ranking,
        })
        .unwrap();

    if rows.len() < 1 {
        return Err(Error::TargetNotFound);
    }

    let response = (StatusCode::OK, Json(rows));

    Ok(response)
}
