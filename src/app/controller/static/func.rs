use super::*;

static UPLOAD_HTML: &str = include_str!("../../../../static/html/upload.html");

async fn get_file_full_path(req_file_path: &str) -> String {
    let now: chrono::DateTime<chrono::Local> = chrono::Local::now();
    let year: String = now.format("%Y").to_string();
    let month: String = now.format("%m").to_string();
    let day: String = now.format("%d").to_string();
    let hour: String = now.format("%H").to_string();
    let minute: String = now.format("%M").to_string();
    let full_dir: String = format!(
        "{}/{}/{}/{}/{}/{}",
        UPLOAD_DIR, year, month, day, hour, minute
    );
    let full_path: String = format!("{}{}", full_dir, req_file_path);
    let dir_path: PathBuf = PathBuf::from(&full_dir);
    let _ = tokio::fs::create_dir_all(&dir_path).await;
    full_path
}

fn replace_prefix_with_hash(input: &str) -> String {
    let (prefix, suffix) = input.rsplit_once('.').unwrap_or((input, ""));
    let mut salt: [u8; 64] = [0u8; 64];
    let _ = rand::rng().try_fill_bytes(&mut salt);
    let mut hasher = Sha256::new();
    hasher.update(prefix.as_bytes());
    hasher.update(salt);
    let hash = hex::encode(hasher.finalize());
    format!("{}.{}", hash, suffix)
}

pub async fn handle(ctx: Context) {
    let dir: String = ctx.get_route_param(DIR_KEY).await.unwrap_or_default();
    let file: String = ctx.get_route_param(FILE_KEY).await.unwrap_or_default();
    if dir.is_empty() || file.is_empty() {
        return;
    }
    let path: String = format!("{}/{}", dir, file);
    ctx.set_response_header(CONTENT_TYPE, content_type_charset(TEXT_HTML, UTF8))
        .await
        .set_response_body(UPLOAD_HTML)
        .await;
}
