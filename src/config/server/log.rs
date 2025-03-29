use crate::*;

pub async fn log_dir(server: &mut Server) {
    server.log_dir("./logs").await;
}

pub async fn log_size(server: &mut Server) {
    server.log_size(100_024_000).await;
}
