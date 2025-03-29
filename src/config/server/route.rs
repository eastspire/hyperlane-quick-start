use crate::*;

pub async fn route(server: &mut Server) {
    server.route("/", app::controller::root::func::root).await;
    server
        .route("/index", app::controller::index::func::index)
        .await;
    server
        .route("/insert", app::controller::mysql::func::insert)
        .await;
    server
        .route(
            "/favicon.ico",
            app::controller::favicon_ico::func::favicon_ico,
        )
        .await;
}
