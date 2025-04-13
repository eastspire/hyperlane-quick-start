use super::*;
use crate::*;

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

    let upload_dir: String = format!("{UPLOAD_DIR}/{file_id}");
    let upload_strategy: ChunkStrategy = ChunkStrategy::new(&upload_dir, |a, b| format!("{a}.{b}"));
    match upload_strategy
        .handle(&file_name, &chunk_data, &file_id, chunk_index, total_chunks)
        .await
    {
        Ok(_) => {
            let _ = ctx.set_response_body(OK).await;
        }
        Err(error) => {
            let _ = ctx.set_response_body(error).await;
        }
    }
}
