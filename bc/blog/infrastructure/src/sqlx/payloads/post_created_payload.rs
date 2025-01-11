use blog_domain::events::PostCreatedEvent;
use serde::{Deserialize, Serialize};
use shared_kernel::domain::date::Date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCreatedPayload {
    pub post_id: String,
    pub title: String,
    pub content: String,
}

impl PostCreatedPayload {
    pub fn into_event(self, version: i64, occurred_on: Date) -> PostCreatedEvent {
        PostCreatedEvent {
            post_id: self.post_id.into(),
            title: self.title,
            content: self.content,
            version,
            occurred_on,
        }
    }
}

impl From<&PostCreatedEvent> for PostCreatedPayload {
    fn from(event: &PostCreatedEvent) -> Self {
        Self {
            post_id: event.post_id.to_string(),
            title: event.title.to_string(),
            content: event.content.to_string(),
        }
    }
}

// impl From<PostCreatedSerializer> for PostCreatedEvent {
//     fn from(event: PostCreatedSerializer) -> Self {
//         Self {
//             post_id: event.post_id.into(),
//             title: event.title,
//             content: event.content,
//             occurred_on
//         }
//     }
// }
