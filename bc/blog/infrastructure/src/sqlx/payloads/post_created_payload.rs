use blog_domain::events::PostCreated;
use serde::{Deserialize, Serialize};
use shared_kernel::domain::date::Date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCreatedPayload {
    pub post_id: String,
    pub title: String,
    pub content: String,
}

impl PostCreatedPayload {
    pub fn into_event(self, version: i64, occurred_on: Date) -> PostCreated {
        PostCreated {
            aggregate_id: self.post_id.into(),
            title: self.title,
            content: self.content,
            version,
            occurred_on,
        }
    }
}

impl From<&PostCreated> for PostCreatedPayload {
    fn from(event: &PostCreated) -> Self {
        Self {
            post_id: event.aggregate_id.to_string(),
            title: event.title.to_string(),
            content: event.content.to_string(),
        }
    }
}
