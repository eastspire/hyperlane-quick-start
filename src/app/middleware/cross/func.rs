use crate::*;

pub async fn cross(ctx: Context) {
    let mut ctx: RwLockWriteInnerContext = ctx.get_write_lock().await;
    let response: &mut Response = ctx.get_mut_response();
    response
        .set_header(ACCESS_CONTROL_ALLOW_ORIGIN, ANY)
        .set_header(ACCESS_CONTROL_ALLOW_METHODS, GET_POST_OPTIONS)
        .set_header(ACCESS_CONTROL_ALLOW_HEADERS, ANY)
        .set_header(
            CONTENT_TYPE,
            format!("{}; {}", APPLICATION_JSON, CHARSET_UTF_8),
        );
}
