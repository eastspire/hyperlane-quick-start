use super::*;

#[derive(Debug, Serialize)]
pub(crate) struct UploadResponse<'a> {
    pub(super) code: i32,
    pub(super) url: &'a str,
    pub(super) name: &'a str,
    pub(super) msg: &'a str,
    pub(super) dir: &'a str,
}
