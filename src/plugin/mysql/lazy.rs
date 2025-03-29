use super::func::init_db_connection;
use crate::*;

pub static DB: Lazy<MySqlPool> = Lazy::new(|| block_on(async { init_db_connection().await }));
