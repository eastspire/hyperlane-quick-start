use crate::{tokio::time::sleep, *};
use std::time::Duration;

pub async fn root(controller_data: ControllerData) {
    let _ = controller_data
        .set_response_header(CONTENT_TYPE, TEXT_EVENT_STREAM)
        .await
        .send_response(200, "")
        .await;
    for i in 0..10 {
        let _ = controller_data
            .send_response_body(format!("data: {}{}", i, HTTP_DOUBLE_BR))
            .await;
        sleep(Duration::from_secs(1)).await;
    }
    let _ = controller_data.close().await;
}
