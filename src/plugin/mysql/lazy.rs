use hyperlane::{once_cell::sync::Lazy, tokio::sync::RwLock};
use sqlx::{MySql, Pool};
use std::sync::Arc;

pub static DB: Lazy<Arc<RwLock<Option<Pool<MySql>>>>> = Lazy::new(|| Arc::new(RwLock::new(None)));
