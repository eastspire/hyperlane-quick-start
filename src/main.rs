pub(crate) mod app;
pub(crate) mod config;
pub(crate) mod init;
pub(crate) mod plugin;
pub(crate) use hyperlane::{futures::executor::block_on, once_cell::sync::Lazy, *};
pub(crate) use sqlx::{
    mysql::MySqlPoolOptions,
    {query, MySqlPool},
};

#[tokio::main]
async fn main() {
    init::server::func::run().await;
}
