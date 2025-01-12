use blog_domain::events::PostContentEdited;
use serde::{Deserialize, Serialize};
use shared_kernel::domain::date::Date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostContentEditedPayload {
    pub post_id: String,
    pub content: String,
}

impl PostContentEditedPayload {
    pub fn into_event(self, version: i64, occurred_on: Date) -> PostContentEdited {
        PostContentEdited {
            aggregate_id: self.post_id.into(),
            content: self.content,
            version,
            occurred_on,
        }
    }
}

impl From<&PostContentEdited> for PostContentEditedPayload {
    fn from(event: &PostContentEdited) -> Self {
        Self {
            post_id: event.aggregate_id.to_string(),
            content: event.content.to_string(),
        }
    }
}
