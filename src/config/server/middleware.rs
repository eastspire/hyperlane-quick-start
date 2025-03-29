use crate::*;

pub async fn middleware(server: &mut Server) {
    server
        .request_middleware(app::middleware::cross::func::cross)
        .await;
    server
        .request_middleware(app::middleware::response_header::func::response_header)
        .await;
    server
        .request_middleware(app::middleware::client::func::client)
        .await;
}
