use std::any::Any;

use shared_kernel::domain::{
    date::Date,
    domain_event::{IApplyDomainEvent, IDomainEvent},
};

use crate::{aggregate_root::PostAggregate, value_objects::post_id::PostId};

use super::PostAggregateEvent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostTitleChangedEvent {
    pub post_id: PostId,
    pub title: String,
    pub occurred_on: Date,
    pub version: i64,
}

impl IApplyDomainEvent<PostAggregate> for PostTitleChangedEvent {
    fn apply_to(&self, aggregate: &mut PostAggregate) {
        aggregate.post.title = self.title.clone();
        aggregate.version = self.version;
    }
}

impl IDomainEvent for PostTitleChangedEvent {
    fn event_type(&self) -> &'static str {
        "PostTitleChanged"
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

impl From<PostTitleChangedEvent> for PostAggregateEvent {
    fn from(event: PostTitleChangedEvent) -> Self {
        PostAggregateEvent::PostTitleChanged(event)
    }
}
