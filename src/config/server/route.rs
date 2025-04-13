use super::*;
use app::controller::{
    self,
    r#static::constant::{DIR_KEY, FILE_KEY},
};

pub async fn register(server: &Server) {
    server.route("/", controller::root::func::handle).await;
    server
        .route("/favicon.ico", controller::favicon_ico::func::favicon_ico)
        .await;
    server
        .route("/upload", controller::upload::func::handle)
        .await;
    server
        .route(
            format!("/static/:{DIR_KEY}/:{FILE_KEY}"),
            controller::r#static::func::handle,
        )
        .await;
    println_success!("Server route initialization completed");
}
