use blog_domain::events::PostTitleChangedEvent;
use serde::{Deserialize, Serialize};
use shared_kernel::domain::date::Date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTitleChangedPayload {
    pub post_id: String,
    pub title: String,
}

impl PostTitleChangedPayload {
    pub fn into_event(self, version: i64, occurred_on: Date) -> PostTitleChangedEvent {
        PostTitleChangedEvent {
            post_id: self.post_id.into(),
            title: self.title,
            version,
            occurred_on,
        }
    }
}

impl From<&PostTitleChangedEvent> for PostTitleChangedPayload {
    fn from(event: &PostTitleChangedEvent) -> Self {
        Self {
            post_id: event.post_id.to_string(),
            title: event.title.to_string(),
        }
    }
}
