mod post_created_event;

pub use post_created_event::PostCreatedEvent;
use serde::{Deserialize, Serialize};
use shared_kernel::domain::{date::Date, domain_event::IDomainEvent};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PostAggregateEvent {
    PostCreated(PostCreatedEvent),
}

impl IDomainEvent for PostAggregateEvent {
    fn event_type(&self) -> &'static str {
        match self {
            PostAggregateEvent::PostCreated(event) => event.event_type(),
        }
    }

    fn event_id(&self) -> String {
        match self {
            PostAggregateEvent::PostCreated(event) => event.event_id(),
        }
    }

    fn occurred_on(&self) -> Date {
        match self {
            PostAggregateEvent::PostCreated(event) => event.occurred_on(),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn version(&self) -> i64 {
        match self {
            PostAggregateEvent::PostCreated(event) => event.version(),
        }
    }
}

impl From<PostCreatedEvent> for PostAggregateEvent {
    fn from(event: PostCreatedEvent) -> Self {
        PostAggregateEvent::PostCreated(event)
    }
}
