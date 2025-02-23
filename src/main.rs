pub(crate) mod app;
pub(crate) mod config;
pub(crate) mod init;
pub(crate) mod plugin;
pub(crate) use hyperlane::*;
pub(crate) use sqlx::{
    mysql::MySqlPoolOptions,
    {query, MySql, MySqlPool, Pool},
};
pub(crate) use std::sync::Arc;

#[tokio::main]
async fn main() {
    init::server::func::run().await;
}
