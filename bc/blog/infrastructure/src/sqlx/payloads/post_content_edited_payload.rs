use blog_domain::events::PostContentEditedEvent;
use serde::{Deserialize, Serialize};
use shared_kernel::domain::date::Date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostContentEditedPayload {
    pub post_id: String,
    pub content: String,
}

impl PostContentEditedPayload {
    pub fn into_event(self, version: i64, occurred_on: Date) -> PostContentEditedEvent {
        PostContentEditedEvent {
            post_id: self.post_id.into(),
            content: self.content,
            version,
            occurred_on,
        }
    }
}

impl From<&PostContentEditedEvent> for PostContentEditedPayload {
    fn from(event: &PostContentEditedEvent) -> Self {
        Self {
            post_id: event.post_id.to_string(),
            content: event.content.to_string(),
        }
    }
}
