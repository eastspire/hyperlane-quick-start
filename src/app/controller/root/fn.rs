use super::*;

static UPLOAD_HTML: &str = include_str!("../../../../resources/static/html/upload.html");

pub async fn handle(ctx: Context) {
    ctx.set_response_header(CONTENT_TYPE, content_type_charset(TEXT_HTML, UTF8))
        .await
        .set_response_body(UPLOAD_HTML)
        .await;
}
