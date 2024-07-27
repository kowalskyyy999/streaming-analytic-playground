use std::{collections::HashMap, sync::{Arc, Mutex, RwLock}};
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub storage: Arc<Mutex<RwLock<Storage>>>
}

#[derive(Debug)]
pub struct Storage {
    pub pool: PgPool,
    pub cache: Arc<Mutex<HashMap<String, RwLock<Buffer>>>>,
}

#[derive(Debug, Default, Clone)]
pub struct Buffer {
    pub num: i32,
    pub start_time: String
}
