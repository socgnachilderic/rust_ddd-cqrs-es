use shared_kernel::domain::{date::Date, domain_event::IApplyDomainEvent, DomainEvent};

use crate::{aggregate_root::PostAggregate, value_objects::post_id::PostId};

use super::PostAggregateEvent;

#[derive(Debug, Clone, PartialEq, Eq, DomainEvent)]
pub struct PostTitleChanged {
    pub aggregate_id: PostId,
    pub title: String,
    pub occurred_on: Date,
    pub version: i64,
}

impl IApplyDomainEvent<PostAggregate> for PostTitleChanged {
    fn apply_to(&self, aggregate: &mut PostAggregate) {
        aggregate.post.title = self.title.clone();
        aggregate.version = self.version;
    }
}

impl From<PostTitleChanged> for PostAggregateEvent {
    fn from(event: PostTitleChanged) -> Self {
        PostAggregateEvent::PostTitleChanged(event)
    }
}
