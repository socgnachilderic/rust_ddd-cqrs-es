mod post_content_edited_payload;
mod post_created_payload;
mod post_title_changed_payload;

use blog_domain::events::PostAggregateEvent;
use post_content_edited_payload::PostContentEditedPayload;
use post_created_payload::PostCreatedPayload;
use post_title_changed_payload::PostTitleChangedPayload;

use serde::{Deserialize, Serialize};
use shared_kernel::domain::date::Date;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PostAggregateEventPayload {
    PostCreated(PostCreatedPayload),
    PostTitleChanged(PostTitleChangedPayload),
    PostContentEdited(PostContentEditedPayload),
}

impl PostAggregateEventPayload {
    pub fn into_event(self, version: i64, occurred_on: Date) -> PostAggregateEvent {
        match self {
            PostAggregateEventPayload::PostCreated(payload) => {
                PostAggregateEvent::PostCreated(payload.into_event(version, occurred_on))
            }
            PostAggregateEventPayload::PostTitleChanged(payload) => {
                PostAggregateEvent::PostTitleChanged(payload.into_event(version, occurred_on))
            }
            PostAggregateEventPayload::PostContentEdited(payload) => {
                PostAggregateEvent::PostContentEdited(payload.into_event(version, occurred_on))
            }
        }
    }
}

impl From<&PostAggregateEvent> for PostAggregateEventPayload {
    fn from(event: &PostAggregateEvent) -> Self {
        match event {
            PostAggregateEvent::PostCreated(event) => Self::PostCreated(event.into()),
            PostAggregateEvent::PostTitleChanged(event) => Self::PostTitleChanged(event.into()),
            PostAggregateEvent::PostContentEdited(event) => Self::PostContentEdited(event.into()),
        }
    }
}
