use super::*;

static UPLOAD_HTML: &str = include_str!("../../../../resources/static/html/upload.html");

pub async fn handle(ctx: Context) {
    let html: String = INDEX_HTML.replace("{{ time }}", &time());
    let _ = ctx.set_response_body(html).await;
}
