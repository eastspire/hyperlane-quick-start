use super::*;

static UPLOAD_HTML: &str = include_str!("../../../../resources/static/html/upload.html");

pub async fn handle(ctx: Context) {
    let _ = ctx
        .set_response_status_code(200)
        .await
        .set_response_body(UPLOAD_HTML)
        .await;
}
