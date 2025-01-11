mod post_content_edited_event;
mod post_created_event;
mod post_title_changed_event;

pub use post_content_edited_event::PostContentEditedEvent;
pub use post_created_event::PostCreatedEvent;
pub use post_title_changed_event::PostTitleChangedEvent;
use shared_kernel::domain::{
    date::Date,
    domain_event::{IApplyDomainEvent, IDomainEvent},
};

use crate::aggregate_root::PostAggregate;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PostAggregateEvent {
    PostCreated(PostCreatedEvent),
    PostTitleChanged(PostTitleChangedEvent),
    PostContentEdited(PostContentEditedEvent),
}

impl IDomainEvent for PostAggregateEvent {
    fn event_type(&self) -> &'static str {
        match self {
            PostAggregateEvent::PostCreated(event) => event.event_type(),
            PostAggregateEvent::PostTitleChanged(event) => event.event_type(),
            PostAggregateEvent::PostContentEdited(event) => event.event_type(),
        }
    }

    fn aggregate_id(&self) -> String {
        match self {
            PostAggregateEvent::PostCreated(event) => event.aggregate_id(),
            PostAggregateEvent::PostTitleChanged(event) => event.aggregate_id(),
            PostAggregateEvent::PostContentEdited(event) => event.aggregate_id(),
        }
    }

    fn occurred_on(&self) -> Date {
        match self {
            PostAggregateEvent::PostCreated(event) => event.occurred_on(),
            PostAggregateEvent::PostTitleChanged(event) => event.occurred_on(),
            PostAggregateEvent::PostContentEdited(event) => event.occurred_on(),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn version(&self) -> i64 {
        match self {
            PostAggregateEvent::PostCreated(event) => event.version(),
            PostAggregateEvent::PostTitleChanged(event) => event.version(),
            PostAggregateEvent::PostContentEdited(event) => event.version(),
        }
    }
}

impl IApplyDomainEvent<PostAggregate> for PostAggregateEvent {
    fn apply_to(&self, aggregate: &mut PostAggregate) {
        match self {
            PostAggregateEvent::PostCreated(event) => event.apply_to(aggregate),
            PostAggregateEvent::PostTitleChanged(event) => event.apply_to(aggregate),
            PostAggregateEvent::PostContentEdited(event) => event.apply_to(aggregate),
        }
    }
}
