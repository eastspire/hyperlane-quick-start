use crate::*;
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{BufWriter, Write},
    path::Path,
    sync::Arc,
};
use tokio::sync::{RwLock, RwLockWriteGuard};

pub trait UploadStrategy: Send + Sync {
    async fn handle_upload(
        &self,
        ctx: &Context,
        file_id: String,
        chunk_index: usize,
        total_chunks: usize,
        file_name: String,
        chunk_data: Vec<u8>,
    ) -> Result<String, String>;
}

pub struct ChunkUploadStrategy {
    uploading_files: Arc<RwLock<HashMap<String, Vec<bool>>>>,
    upload_dir: String,
}

impl ChunkUploadStrategy {
    pub fn new(upload_dir: String) -> Self {
        Self {
            uploading_files: Arc::new(RwLock::new(HashMap::new())),
            upload_dir,
        }
    }

    async fn save_chunk(&self, chunk_path: &str, chunk_data: &[u8]) -> Result<(), String> {
        match File::create(chunk_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(chunk_data) {
                    return Err(format!("Failed to write chunk: {}", e));
                }
                Ok(())
            }
            Err(e) => Err(format!("Failed to create chunk file: {}", e)),
        }
    }

    async fn merge_chunks(
        &self,
        file_id: &str,
        file_name: &str,
        total_chunks: usize,
    ) -> Result<String, String> {
        let final_path: String = format!("{}/{}", self.upload_dir, file_name);
        let output_file: File = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&final_path)
            .map_err(|e| format!("Failed to create output file: {}", e))?;

        let mut writer: BufWriter<File> = BufWriter::new(output_file);

        for i in 0..total_chunks {
            let chunk_path: String = format!("{}/{}_{}", self.upload_dir, file_id, i);
            let chunk_data: Vec<u8> =
                fs::read(&chunk_path).map_err(|e| format!("Failed to read chunk {}: {}", i, e))?;

            writer
                .write_all(&chunk_data)
                .map_err(|e| format!("Failed to write to output file: {}", e))?;

            let _ = fs::remove_file(&chunk_path);
        }

        Ok(format!("File {} uploaded successfully", file_name))
    }
}

impl UploadStrategy for ChunkUploadStrategy {
    async fn handle_upload(
        &self,
        _ctx: &Context,
        file_id: String,
        chunk_index: usize,
        total_chunks: usize,
        file_name: String,
        chunk_data: Vec<u8>,
    ) -> Result<String, String> {
        if !Path::new(&self.upload_dir).exists() {
            fs::create_dir_all(&self.upload_dir)
                .map_err(|e| format!("Failed to create upload directory: {}", e))?;
        }

        let chunk_path: String = format!("{}/{}_{}", self.upload_dir, file_id, chunk_index);
        self.save_chunk(&chunk_path, &chunk_data).await?;

        let mut uploading_files: RwLockWriteGuard<'_, HashMap<String, Vec<bool>>> =
            self.uploading_files.write().await;
        let chunks_status: &mut Vec<bool> = uploading_files
            .entry(file_id.clone())
            .or_insert_with(|| vec![false; total_chunks]);

        if chunks_status.len() != total_chunks {
            *chunks_status = vec![false; total_chunks];
        }

        if chunk_index < chunks_status.len() {
            chunks_status[chunk_index] = true;
        }

        let all_chunks_uploaded: bool = chunks_status.iter().all(|&status| status);

        if all_chunks_uploaded {
            uploading_files.remove(&file_id);
            drop(uploading_files);
            self.merge_chunks(&file_id, &file_name, total_chunks).await
        } else {
            Ok(format!(
                "Chunk {} of {} uploaded successfully",
                chunk_index + 1,
                total_chunks
            ))
        }
    }
}
