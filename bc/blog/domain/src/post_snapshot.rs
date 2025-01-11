use serde::{Deserialize, Serialize};
use shared_kernel::domain::snapshot::ISnapshot;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSnapshot {
    pub id: String,
    pub title: String,
    pub content: String,
    pub version: i64,
}

impl ISnapshot for PostSnapshot {}
