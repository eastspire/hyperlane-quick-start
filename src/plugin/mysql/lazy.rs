use crate::*;
use hyperlane::{once_cell::sync::Lazy, tokio::sync::RwLock};

pub static DB: Lazy<ArcRwLock<Option<MySqlPool>>> = Lazy::new(|| Arc::new(RwLock::new(None)));
