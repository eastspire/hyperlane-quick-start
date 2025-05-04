use super::*;

impl FileChunkData {
    pub fn new(
        file_id: String,
        file_name: String,
        chunk_index: usize,
        total_chunks: usize,
        base_file_dir: String,
    ) -> Self {
        Self {
            file_id,
            chunk_index,
            total_chunks,
            file_name,
            base_file_dir,
        }
    }
}

impl<'a> Default for UploadResponse<'a> {
    fn default() -> Self {
        Self {
            code: 100,
            url: "",
            name: "",
            msg: "",
            base_file_dir: "",
        }
    }
}
