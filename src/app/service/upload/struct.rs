use super::*;

#[derive(Debug, Default, Lombok, Clone)]
pub(crate) struct FileChunkData {
    pub(super) file_id: String,
    pub(super) file_name: String,
    pub(super) chunk_index: usize,
    pub(super) total_chunks: usize,
    pub(super) base_file_dir: String,
}

#[derive(Debug, Serialize, Lombok)]
pub(crate) struct UploadResponse<'a> {
    pub(crate) code: i32,
    pub(crate) url: &'a str,
    pub(crate) name: &'a str,
    pub(crate) msg: &'a str,
    pub(crate) base_file_dir: &'a str,
}
