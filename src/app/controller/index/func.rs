use crate::*;

pub async fn index(ctx: Context) {
    let send_res: ResponseResult = ctx.send_response(200, "hello hyperlane => /index").await;
    ctx.log_info(&format!("Response result => {:?}", send_res), log_handler)
        .await;
}
