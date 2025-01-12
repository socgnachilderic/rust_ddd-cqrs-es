mod post_content_edited;
mod post_created;
mod post_title_changed;

pub use post_content_edited::PostContentEdited;
pub use post_created::PostCreated;
pub use post_title_changed::PostTitleChanged;
use shared_kernel::domain::{domain_event::IApplyDomainEvent, DomainEvent};

use crate::aggregate_root::PostAggregate;

#[derive(Debug, Clone, Eq, PartialEq, DomainEvent)]
pub enum PostAggregateEvent {
    PostCreated(PostCreated),
    PostTitleChanged(PostTitleChanged),
    PostContentEdited(PostContentEdited),
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
