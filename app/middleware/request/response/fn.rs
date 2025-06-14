use super::*;

pub async fn response_header(ctx: Context) {
    let socket_addr_string: String = ctx.get_socket_addr_or_default_string().await;
    let content_type: String = ContentType::format_content_type_with_charset(TEXT_HTML, UTF8);
    ctx.set_response_header(SERVER, HYPERLANE)
        .await
        .set_response_header(CONNECTION, KEEP_ALIVE)
        .await
        .set_response_header(CONTENT_TYPE, content_type)
        .await
        .set_response_header(DATE, gmt())
        .await
        .set_response_header("SocketAddr", socket_addr_string)
        .await;
}

pub async fn response_status_code(ctx: Context) {
    ctx.set_response_status_code(404).await;
}

pub async fn response_body(ctx: Context) {
    ctx.set_response_body(NOT_FOUND_HTML).await;
}
