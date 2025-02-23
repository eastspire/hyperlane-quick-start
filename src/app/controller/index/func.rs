use crate::*;

pub async fn index(arc_lock_controller_data: ArcRwLockControllerData) {
    let send_res: ResponseResult = arc_lock_controller_data
        .send_response(200, "hello hyperlane => /index")
        .await;
    let controller_data: ControllerData = arc_lock_controller_data.get_controller_data().await;
    controller_data.get_log().info(
        format!("Response result => {:?}", send_res),
        log_debug_format_handler,
    );
}
