use super::*;

pub(crate) async fn get_register_file_chunk_data<'a>(ctx: &'a Context) -> OptionFileChunkData {
    let file_id: String = match ctx.get_request_header(CHUNKIFY_FILE_ID_HEADER).await {
        Some(id) => id,
        None => {
            let _ = ctx
                .set_response_body(ChunkStrategyError::MissingFileId)
                .await;
            return None;
        }
    };
    let total_chunks: usize = match ctx.get_request_header(CHUNKIFY_TOTAL_CHUNKS_HEADER).await {
        Some(total) => match total.parse::<usize>() {
            Ok(t) => t,
            Err(_) => {
                let _ = ctx
                    .set_response_body(ChunkStrategyError::InvalidTotalChunks)
                    .await;
                return None;
            }
        },
        None => {
            let _ = ctx
                .set_response_body(ChunkStrategyError::MissingTotalChunks)
                .await;
            return None;
        }
    };
    let file_name: String = match ctx.get_request_header(CHUNKIFY_FILE_NAME_HEADER).await {
        Some(name) => urlencoding::decode(&name).unwrap_or_default().into_owned(),
        None => {
            let _ = ctx
                .set_response_body(ChunkStrategyError::MissingFileName)
                .await;
            return None;
        }
    };
    let base_file_dir: String = match ctx.get_request_header(CHUNKIFY_DIRECTORY_HEADER).await {
        Some(encode_dir) => {
            let decode_dir: String = urlencoding::decode(&encode_dir)
                .unwrap_or_default()
                .into_owned();
            if decode_dir.is_empty()
                || decode_dir.contains("../")
                || !decode_dir.chars().all(|c| c.is_ascii_digit() || c == '/')
            {
                get_base_file_dir()
            } else {
                decode_dir
            }
        }
        None => get_base_file_dir(),
    };
    Some(FileChunkData::new(
        file_id,
        file_name,
        0,
        total_chunks,
        base_file_dir,
    ))
}

pub(crate) async fn get_save_file_chunk_data<'a>(ctx: &'a Context) -> OptionFileChunkData {
    let mut data: FileChunkData = get_merge_file_chunk_data(ctx).await?;
    let chunk_index: usize = match ctx.get_request_header(CHUNKIFY_CHUNK_INDEX_HEADER).await {
        Some(idx) => match idx.parse::<usize>() {
            Ok(i) => i,
            Err(_) => {
                let _ = ctx
                    .set_response_body(ChunkStrategyError::InvalidChunkIndex)
                    .await;
                return None;
            }
        },
        None => {
            let _ = ctx
                .set_response_body(ChunkStrategyError::MissingChunkIndex)
                .await;
            return None;
        }
    };
    data.set_chunk_index(chunk_index);
    Some(data)
}

pub(crate) fn add_file_id_map(data: &FileChunkData) {
    FILE_ID_MAP.insert(data.file_id.to_owned(), data.clone());
}

pub(crate) fn remove_file_id_map(file_id: &str) {
    FILE_ID_MAP.remove(file_id);
}

pub(crate) async fn get_merge_file_chunk_data<'a>(ctx: &Context) -> OptionFileChunkData {
    let key: String = ctx
        .get_request_header(CHUNKIFY_FILE_ID_HEADER)
        .await
        .unwrap_or_default();
    let data_opt = FILE_ID_MAP.get(&key);
    if data_opt.is_none() {
        return None;
    }
    Some(data_opt.unwrap().clone())
}
