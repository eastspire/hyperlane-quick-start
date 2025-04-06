use crate::*;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

// 存储临时文件块的目录
const UPLOAD_DIR: &str = "uploads";

// 用于存储上传中的文件信息
static UPLOADING_FILES: once_cell::sync::Lazy<
    Arc<Mutex<std::collections::HashMap<String, Vec<bool>>>>,
> = once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(std::collections::HashMap::new())));

pub async fn handle(ctx: Context) {
    // 确保上传目录存在
    if !Path::new(UPLOAD_DIR).exists() {
        let _ = fs::create_dir_all(UPLOAD_DIR);
    }

    // 从请求头获取文件信息
    let file_id = match ctx.get_request_header("x-file-id").await {
        Some(id) => id,
        None => {
            let _ = ctx.send_response(400, b"Missing X-File-Id header").await;
            return;
        }
    };

    let chunk_index = match ctx.get_request_header("x-chunk-index").await {
        Some(idx) => match idx.parse::<usize>() {
            Ok(i) => i,
            Err(_) => {
                let _ = ctx
                    .send_response(400, b"Invalid X-Chunk-Index header")
                    .await;
                return;
            }
        },
        None => {
            let _ = ctx
                .send_response(400, b"Missing X-Chunk-Index header")
                .await;
            return;
        }
    };

    let total_chunks = match ctx.get_request_header("x-total-chunks").await {
        Some(total) => match total.parse::<usize>() {
            Ok(t) => t,
            Err(_) => {
                let _ = ctx
                    .send_response(400, b"Invalid X-Total-Chunks header")
                    .await;
                return;
            }
        },
        None => {
            let _ = ctx
                .send_response(400, b"Missing X-Total-Chunks header")
                .await;
            return;
        }
    };

    let file_name = match ctx.get_request_header("x-file-name").await {
        Some(name) => name,
        None => {
            let _ = ctx.send_response(400, b"Missing X-File-Name header").await;
            return;
        }
    };

    // 获取请求体（文件块数据）
    let chunk_data: Vec<u8> = ctx.get_request_body().await;
    if chunk_data.is_empty() {
        let _ = ctx.send_response(400, b"Empty chunk data").await;
        return;
    }

    // 保存文件块
    let chunk_path = format!("{}/{}_{}", UPLOAD_DIR, file_id, chunk_index);
    match File::create(&chunk_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(&chunk_data) {
                let _ = ctx
                    .send_response(500, format!("Failed to write chunk: {}", e).as_bytes())
                    .await;
                return;
            }
        }
        Err(e) => {
            let _ = ctx
                .send_response(
                    500,
                    format!("Failed to create chunk file: {}", e).as_bytes(),
                )
                .await;
            return;
        }
    }

    // 更新文件上传状态
    let mut uploading_files = UPLOADING_FILES.lock().await;
    let chunks_status = uploading_files
        .entry(file_id.clone())
        .or_insert_with(|| vec![false; total_chunks]);

    // 如果这是第一个块，确保状态数组大小正确
    if chunks_status.len() != total_chunks {
        *chunks_status = vec![false; total_chunks];
    }

    // 标记当前块为已上传
    if chunk_index < chunks_status.len() {
        chunks_status[chunk_index] = true;
    }

    // 检查是否所有块都已上传
    let all_chunks_uploaded = chunks_status.iter().all(|&status| status);

    // 如果所有块都已上传，合并文件
    if all_chunks_uploaded {
        // 从上传状态中移除该文件
        uploading_files.remove(&file_id);
        drop(uploading_files); // 释放锁

        // 合并文件块
        let final_path = format!("{}/{}", UPLOAD_DIR, file_name);
        match OpenOptions::new()
            .create(true)
            .write(true)
            .open(&final_path)
        {
            Ok(mut output_file) => {
                let mut success = true;

                // 按顺序读取并写入每个块
                for i in 0..total_chunks {
                    let chunk_path = format!("{}/{}_{}", UPLOAD_DIR, file_id, i);
                    match fs::read(&chunk_path) {
                        Ok(data) => {
                            if let Err(e) = output_file.write_all(&data) {
                                let _ = ctx
                                    .send_response(
                                        500,
                                        format!("Failed to write to output file: {}", e).as_bytes(),
                                    )
                                    .await;
                                success = false;
                                break;
                            }
                            // 删除临时块文件
                            let _ = fs::remove_file(&chunk_path);
                        }
                        Err(e) => {
                            let _ = ctx
                                .send_response(
                                    500,
                                    format!("Failed to read chunk {}: {}", i, e).as_bytes(),
                                )
                                .await;
                            success = false;
                            break;
                        }
                    }
                }

                if success {
                    let _ = ctx
                        .send_response(
                            200,
                            format!("File {} uploaded successfully", file_name).as_bytes(),
                        )
                        .await;
                }
            }
            Err(e) => {
                let _ = ctx
                    .send_response(
                        500,
                        format!("Failed to create output file: {}", e).as_bytes(),
                    )
                    .await;
            }
        }
    } else {
        // 如果还有块未上传，返回当前块上传成功
        let _ = ctx
            .send_response(
                200,
                format!(
                    "Chunk {} of {} uploaded successfully",
                    chunk_index + 1,
                    total_chunks
                )
                .as_bytes(),
            )
            .await;
    }
}
