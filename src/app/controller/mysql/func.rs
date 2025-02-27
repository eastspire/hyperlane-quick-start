use crate::plugin::mysql::func::insert_record;
use hyperlane::{serde_json::json, *};

pub async fn insert(arc_lock_controller_data: ArcRwLockControllerData) {
    let response_data: String = json!({
        "code": 1,
        "msg": "success",
    })
    .to_string();
    let send_result: ResponseResult = arc_lock_controller_data
        .set_response_header(CONTENT_TYPE, APPLICATION_JSON)
        .await
        .send_response(200, response_data)
        .await;
    let controller_data: ControllerData = arc_lock_controller_data.get_controller_data().await;
    controller_data.get_log().info(
        format!("Response result => {:?}", send_result),
        log_debug_handler,
    );
    let request: String = controller_data.get_request().to_string();
    let response: String = controller_data.get_response().to_string();
    insert_record(&request, &response).await;
}
