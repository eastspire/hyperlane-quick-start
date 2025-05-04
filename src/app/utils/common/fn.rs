use super::*;

pub(crate) async fn set_common_success_response_body<'a>(ctx: &'a Context, url: &'a str) {
    let mut data: UploadResponse<'_> = UploadResponse::default();
    data.set_code(200).set_msg(OK).set_url(url);
    let data_json: String = serde_json::to_string(&data).unwrap_or_default();
    let _ = ctx.set_response_body(data_json).await;
}

pub(crate) async fn set_common_error_response_body<'a>(ctx: &'a Context, error: String) {
    let mut data: UploadResponse<'_> = UploadResponse::default();
    data.set_msg(&error);
    let data_json: String = serde_json::to_string(&data).unwrap_or_default();
    let _ = ctx.set_response_body(data_json).await;
}
