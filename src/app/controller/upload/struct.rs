use super::*;

#[derive(Debug, Serialize)]
pub(crate) struct UploadResponse<'a> {
    pub(super) code: i32,
    pub(super) data: String,
    pub(super) msg: &'a str,
}
