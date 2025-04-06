use crate::*;
use app::controller::*;

pub async fn register(server: &Server) {
    server.route("/", root::func::handle).await;
    server.route("/websocket", websocket::func::handle).await;
    server
        .route("/favicon.ico", favicon_ico::func::favicon_ico)
        .await;
    server.route("/upload", upload::func::handle).await;
    server.route("/upload-page", upload_page::func::handle).await;
    println_success!("Server route initialization completed");
}
