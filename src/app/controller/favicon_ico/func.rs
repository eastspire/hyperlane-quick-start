use crate::*;

pub async fn favicon_ico(ctx: Context) {
    let data: Vec<u8> = plugin::logo_img::func::get_logo_img();
    {
        let mut ctx: RwLockWriteInnerContext = ctx.get_write_lock().await;
        let response: &mut Response = ctx.get_mut_response();
        response.set_header(CONTENT_TYPE, IMAGE_PNG);
        response.set_header(CACHE_CONTROL, "public, max-age=3600");
    }
    let send_res: ResponseResult = ctx.send_response(200, data).await;
    ctx.log_info(&format!("Response result => {:?}", send_res), log_handler)
        .await;
}
