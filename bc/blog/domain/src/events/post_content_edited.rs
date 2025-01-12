use shared_kernel::domain::{date::Date, domain_event::IApplyDomainEvent, DomainEvent};

use crate::{aggregate_root::PostAggregate, value_objects::post_id::PostId};

use super::PostAggregateEvent;

#[derive(Debug, Clone, PartialEq, Eq, DomainEvent)]
pub struct PostContentEdited {
    pub aggregate_id: PostId,
    pub content: String,
    pub occurred_on: Date,
    pub version: i64,
}

impl IApplyDomainEvent<PostAggregate> for PostContentEdited {
    fn apply_to(&self, aggregate: &mut PostAggregate) {
        aggregate.post.content = self.content.clone();
        aggregate.version = self.version;
    }
}

impl From<PostContentEdited> for PostAggregateEvent {
    fn from(event: PostContentEdited) -> Self {
        PostAggregateEvent::PostContentEdited(event)
    }
}
