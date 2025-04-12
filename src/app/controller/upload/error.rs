use std::fmt;

#[derive(Debug)]
pub enum UploadError {
    MissingFileId,
    InvalidChunkIndex,
    MissingChunkIndex,
    InvalidTotalChunks,
    MissingTotalChunks,
    MissingFileName,
    EmptyChunkData,
}

impl fmt::Display for UploadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            UploadError::MissingFileId => "Missing X-File-Id header",
            UploadError::InvalidChunkIndex => "Invalid X-Chunk-Index header",
            UploadError::MissingChunkIndex => "Missing X-Chunk-Index header",
            UploadError::InvalidTotalChunks => "Invalid X-Total-Chunks header",
            UploadError::MissingTotalChunks => "Missing X-Total-Chunks header",
            UploadError::MissingFileName => "Missing X-File-Name header",
            UploadError::EmptyChunkData => "Empty chunk data",
        };
        write!(f, "{}", message)
    }
}

impl std::error::Error for UploadError {}

impl From<UploadError> for Vec<u8> {
    fn from(error: UploadError) -> Self {
        error.to_string().into_bytes()
    }
}
