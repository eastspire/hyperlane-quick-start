use super::*;

fn get_base_file_dir() -> String {
    let (year, month, day, hour, minute, _, _, _) = calculate_time();
    let full_dir: String = format!("{}/{}/{}/{}/{}", year, month, day, hour, minute);
    full_dir
}

pub async fn handle(ctx: Context) {
    let file_id: String = match ctx.get_request_header(CHUNKIFY_FILE_ID_HEADER).await {
        Some(id) => id,
        None => {
            let _ = ctx
                .set_response_body(ChunkStrategyError::MissingFileId)
                .await;
            return;
        }
    };

    let chunk_index: usize = match ctx.get_request_header(CHUNKIFY_CHUNK_INDEX_HEADER).await {
        Some(idx) => match idx.parse::<usize>() {
            Ok(i) => i,
            Err(_) => {
                let _ = ctx
                    .set_response_body(ChunkStrategyError::InvalidChunkIndex)
                    .await;
                return;
            }
        },
        None => {
            let _ = ctx
                .set_response_body(ChunkStrategyError::MissingChunkIndex)
                .await;
            return;
        }
    };

    let total_chunks: usize = match ctx.get_request_header(CHUNKIFY_TOTAL_CHUNKS_HEADER).await {
        Some(total) => match total.parse::<usize>() {
            Ok(t) => t,
            Err(_) => {
                let _ = ctx
                    .set_response_body(ChunkStrategyError::InvalidTotalChunks)
                    .await;
                return;
            }
        },
        None => {
            let _ = ctx
                .set_response_body(ChunkStrategyError::MissingTotalChunks)
                .await;
            return;
        }
    };

    let file_name: String = match ctx.get_request_header(CHUNKIFY_FILE_NAME_HEADER).await {
        Some(name) => name,
        None => {
            let _ = ctx
                .set_response_body(ChunkStrategyError::MissingFileName)
                .await;
            return;
        }
    };

    let chunk_data: Vec<u8> = ctx.get_request_body().await;
    if chunk_data.is_empty() {
        let _ = ctx
            .set_response_body(ChunkStrategyError::EmptyChunkData)
            .await;
        return;
    }
    let base_file_dir: String = get_base_file_dir();
    let save_upload_dir: String = format!("{UPLOAD_DIR}/{base_file_dir}/{file_id}");
    let upload_strategy: ChunkStrategy =
        ChunkStrategy::new(&save_upload_dir, |a, b| format!("{a}.{b}"));
    let mut url: String = String::new();
    if chunk_index + 1 == total_chunks {
        let url_encode_dir: String =
            encode(CHARSETS, &format!("{base_file_dir}/{file_id}")).unwrap_or_default();
        let url_encode_dir_file_name: String = encode(CHARSETS, &file_name).unwrap_or_default();
        url = format!("/{url_encode_dir}/{url_encode_dir_file_name}");
    }
    match upload_strategy
        .handle(&file_name, &chunk_data, &file_id, chunk_index, total_chunks)
        .await
    {
        Ok(_) => {
            let data: UploadResponse<'_> = UploadResponse {
                code: 1,
                data: url,
                msg: OK,
            };
            let data_json: String = serde_json::to_string(&data).unwrap_or_default();
            let _ = ctx.set_response_body(data_json).await;
        }
        Err(error) => {
            let data: UploadResponse<'_> = UploadResponse {
                code: 0,
                data: url,
                msg: &error.to_string(),
            };
            let data_json: String = serde_json::to_string(&data).unwrap_or_default();
            let _ = ctx.set_response_body(data_json).await;
        }
    }
}
