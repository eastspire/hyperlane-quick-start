use super::*;
use crate::*;

pub async fn handle(ctx: Context) {
    let file_id: String = match ctx.get_request_header(FILE_ID_HEADER).await {
        Some(id) => id,
        None => {
            let _ = ctx.set_response_body(UploadError::MissingFileId).await;
            return;
        }
    };

    let chunk_index: usize = match ctx.get_request_header(CHUNK_INDEX_HEADER).await {
        Some(idx) => match idx.parse::<usize>() {
            Ok(i) => i,
            Err(_) => {
                let _ = ctx.set_response_body(UploadError::InvalidChunkIndex).await;
                return;
            }
        },
        None => {
            let _ = ctx.set_response_body(UploadError::MissingChunkIndex).await;
            return;
        }
    };

    let total_chunks: usize = match ctx.get_request_header(TOTAL_CHUNKS_HEADER).await {
        Some(total) => match total.parse::<usize>() {
            Ok(t) => t,
            Err(_) => {
                let _ = ctx.set_response_body(UploadError::InvalidTotalChunks).await;
                return;
            }
        },
        None => {
            let _ = ctx.set_response_body(UploadError::MissingTotalChunks).await;
            return;
        }
    };

    let file_name: String = match ctx.get_request_header(FILE_NAME_HEADER).await {
        Some(name) => name,
        None => {
            let _ = ctx.set_response_body(UploadError::MissingFileName).await;
            return;
        }
    };

    let chunk_data: Vec<u8> = ctx.get_request_body().await;
    if chunk_data.is_empty() {
        let _ = ctx.set_response_body(UploadError::EmptyChunkData).await;
        return;
    }

    let upload_strategy: ChunkUploadStrategy = ChunkUploadStrategy::new(UPLOAD_DIR.to_owned());
    match upload_strategy
        .handle_upload(
            &ctx,
            file_id,
            chunk_index,
            total_chunks,
            file_name,
            chunk_data,
        )
        .await
    {
        Ok(message) => {
            let _ = ctx.set_response_body(message).await;
        }
        Err(error) => {
            let _ = ctx.set_response_body(error).await;
        }
    }
}
