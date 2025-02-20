use crate::{plugin::mysql::func::insert_record, *};

pub async fn insert(arc_lock_controller_data: ArcRwLockControllerData) {
    insert_record().await;
    let send_res: ResponseResult =
        send_response(&arc_lock_controller_data, 200, "hello hyperlane => /").await;
    let controller_data: ControllerData = get_controller_data(&arc_lock_controller_data).await;
    controller_data.get_log().info(
        format!("Response result => {:?}", send_res),
        log_debug_handler,
    );
}
