use super::*;

pub async fn handle(ctx: Context) {
    let dir: String = ctx.get_route_param(DIR_KEY).await.unwrap_or_default();
    let file: String = ctx.get_route_param(FILE_KEY).await.unwrap_or_default();
    let decode_dir: String = decode(CHARSET, &dir).unwrap_or_default();
    let decode_file: String = decode(CHARSET, &file).unwrap_or_default();
    if decode_dir.is_empty() || decode_file.is_empty() {
        return;
    }
    let path: String = format!("{}/{}/{}", UPLOAD_DIR, decode_dir, decode_file);
    let data: Vec<u8> = async_read_from_file(&path).await.unwrap_or_default();
    ctx.set_response_header(CONTENT_TYPE, content_type_charset(TEXT_HTML, UTF8))
        .await
        .set_response_body(data)
        .await;
}
