use super::{FileId, LineId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ResourceStatus(FileId, LineId);

impl ResourceStatus {
    #[inline]
    pub fn last_file_id(&self) -> FileId {
        self.0
    }

    #[inline]
    pub fn last_line_id(&self) -> LineId {
        self.1
    }
}
