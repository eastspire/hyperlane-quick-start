use super::*;
use config::r#static::r#const::*;

pub async fn register(server: &Server) {
    server.route("/", controller::root::func::handle).await;
    server
        .route("/favicon.ico", controller::favicon_ico::func::handle)
        .await;
    server
        .route("/upload", controller::upload::func::handle)
        .await;
    server
        .route(
            format!("/{STATIC_ROUTE}/:{DIR_KEY}/:{FILE_KEY}"),
            controller::r#static::func::handle,
        )
        .await;
    println_success!("Server route initialization completed");
}
