use std::any::Any;

use shared_kernel::domain::{
    date::Date,
    domain_event::{IApplyDomainEvent, IDomainEvent},
};

use crate::{aggregate_root::PostAggregate, value_objects::post_id::PostId};

use super::PostAggregateEvent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostContentEditedEvent {
    pub post_id: PostId,
    pub content: String,
    pub occurred_on: Date,
    pub version: i64,
}

impl IApplyDomainEvent<PostAggregate> for PostContentEditedEvent {
    fn apply_to(&self, aggregate: &mut PostAggregate) {
        aggregate.post.content = self.content.clone();
        aggregate.version = self.version;
    }
}

impl IDomainEvent for PostContentEditedEvent {
    fn event_type(&self) -> &'static str {
        "PostContentChanged"
    }
    fn aggregate_id(&self) -> String {
        self.post_id.to_string()
    }

    fn occurred_on(&self) -> Date {
        self.occurred_on.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn version(&self) -> i64 {
        self.version
    }
}

impl From<PostContentEditedEvent> for PostAggregateEvent {
    fn from(event: PostContentEditedEvent) -> Self {
        PostAggregateEvent::PostContentEdited(event)
    }
}
