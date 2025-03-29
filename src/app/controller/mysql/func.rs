use crate::plugin::mysql::func::insert_record;
use hyperlane::{serde_json::json, *};

pub async fn insert(ctx: Context) {
    let response_data: String = json!({
        "code": 1,
        "msg": "success",
    })
    .to_string();
    let send_result: ResponseResult = ctx
        .set_response_header(CONTENT_TYPE, APPLICATION_JSON)
        .await
        .send_response(200, response_data)
        .await;
    ctx.log_info(
        &format!("Response result => {:?}", send_result),
        log_handler,
    )
    .await;
    let request: String = ctx.get_request().await.get_string();
    let response: String = ctx.get_response().await.get_string();
    insert_record(&request, &response).await;
}
