use crate::*;

pub async fn print(server: &Server) {
    server.inner_print(true).await;
}
