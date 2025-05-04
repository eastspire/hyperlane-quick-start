use super::*;

pub async fn register(ctx: Context) {
    let file_chunk_data_opt: OptionFileChunkData = get_register_file_chunk_data(&ctx).await;
    if file_chunk_data_opt.is_none() {
        return;
    }
    let file_chunk_data: FileChunkData = file_chunk_data_opt.unwrap_or_default();
    add_file_id_map(&file_chunk_data);
    let mut data: UploadResponse<'_> = UploadResponse::default();
    data.set_code(200)
        .set_name(file_chunk_data.get_file_name())
        .set_msg(OK)
        .set_base_file_dir(file_chunk_data.get_base_file_dir());
    let data_json: String = serde_json::to_string(&data).unwrap_or_default();
    let _ = ctx.set_response_body(data_json).await;
}

pub async fn save(ctx: Context) {
    let file_chunk_data_opt: OptionFileChunkData = get_save_file_chunk_data(&ctx).await;
    if file_chunk_data_opt.is_none() {
        return;
    }
    let file_chunk_data: FileChunkData = file_chunk_data_opt.unwrap_or_default();
    let file_id: &str = file_chunk_data.get_file_id();
    let file_name: &str = file_chunk_data.get_file_name();
    let chunk_index: &usize = file_chunk_data.get_chunk_index();
    let total_chunks: &usize = file_chunk_data.get_total_chunks();
    let base_file_dir: &str = file_chunk_data.get_base_file_dir();
    let chunk_data: Vec<u8> = ctx.get_request_body().await;
    if chunk_data.is_empty() {
        let _ = ctx
            .set_response_body(ChunkStrategyError::EmptyChunkData)
            .await;
        return;
    }
    let save_upload_dir: String = format!("{UPLOAD_DIR}/{base_file_dir}/{file_id}");
    let upload_strategy: ChunkStrategy = ChunkStrategy::new(
        0,
        &save_upload_dir,
        &file_id,
        &file_name,
        *total_chunks,
        |a, b| format!("{a}.{b}"),
    )
    .unwrap();
    let url: String = String::new();
    let mut data: UploadResponse<'_> = UploadResponse::default();
    match upload_strategy.save_chunk(&chunk_data, *chunk_index).await {
        Ok(_) => {
            data.set_code(200)
                .set_url(&url)
                .set_name(file_name)
                .set_msg(OK)
                .set_base_file_dir(base_file_dir);
            let data_json: String = serde_json::to_string(&data).unwrap_or_default();
            let _ = ctx.set_response_body(data_json).await;
        }
        Err(error) => {
            let msg: String = error.to_string();
            data.set_code(100)
                .set_url(&url)
                .set_name(file_name)
                .set_msg(&msg)
                .set_base_file_dir(base_file_dir);
            let data_json: String = serde_json::to_string(&data).unwrap_or_default();
            let _ = ctx.set_response_body(data_json).await;
        }
    }
}

pub async fn merge(ctx: Context) {
    let file_chunk_data_opt: OptionFileChunkData = get_merge_file_chunk_data(&ctx).await;
    if file_chunk_data_opt.is_none() {
        return;
    }
    let file_chunk_data: FileChunkData = file_chunk_data_opt.unwrap_or_default();
    let file_id: &str = file_chunk_data.get_file_id();
    let file_name: &str = file_chunk_data.get_file_name();
    let total_chunks: &usize = file_chunk_data.get_total_chunks();
    let base_file_dir: &str = file_chunk_data.get_base_file_dir();
    let save_upload_dir: String = format!("{UPLOAD_DIR}/{base_file_dir}/{file_id}");
    let upload_strategy: ChunkStrategy = ChunkStrategy::new(
        0,
        &save_upload_dir,
        &file_id,
        &file_name,
        *total_chunks,
        |a, b| format!("{a}.{b}"),
    )
    .unwrap();
    let url_encode_dir: String =
        encode(CHARSETS, &format!("{base_file_dir}/{file_id}")).unwrap_or_default();
    let url_encode_dir_file_name: String = encode(CHARSETS, &file_name).unwrap_or_default();
    let url: String = format!("/{STATIC_ROUTE}/{url_encode_dir}/{url_encode_dir_file_name}");
    let mut data: UploadResponse<'_> = UploadResponse::default();
    match upload_strategy.merge_chunks().await {
        Ok(_) => {
            remove_file_id_map(&file_id);
            data.set_code(200)
                .set_url(&url)
                .set_name(file_name)
                .set_msg(OK)
                .set_base_file_dir(base_file_dir);
            let data_json: String = serde_json::to_string(&data).unwrap_or_default();
            let _ = ctx.set_response_body(data_json).await;
        }
        Err(error) => {
            let msg: String = error.to_string();
            data.set_code(100)
                .set_url(&url)
                .set_name(file_name)
                .set_msg(&msg)
                .set_base_file_dir(base_file_dir);
            let data_json: String = serde_json::to_string(&data).unwrap_or_default();
            let _ = ctx.set_response_body(data_json).await;
        }
    }
}
