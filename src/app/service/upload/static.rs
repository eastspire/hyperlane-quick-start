use super::*;

pub(super) static FILE_ID_MAP: Lazy<DashMap<String, FileChunkData>> = Lazy::new(|| DashMap::new());
