use super::*;
use crate::app::controller::r#static::{DIR_KEY, FILE_KEY};
use app::controller::*;

pub async fn register(server: &Server) {
    server.route("/", root::func::handle).await;
    server
        .route("/favicon.ico", favicon_ico::func::favicon_ico)
        .await;
    server.route("/upload", upload::func::handle).await;
    server
        .route(
            format!("/static/:{DIR_KEY}/:{FILE_KEY}"),
            r#static::func::handle,
        )
        .await;
    println_success!("Server route initialization completed");
}
