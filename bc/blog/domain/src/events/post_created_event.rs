use std::any::Any;

use serde::{Deserialize, Serialize};
use shared_kernel::domain::{date::Date, domain_event::IDomainEvent};

use crate::value_objects::post_id::PostId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostCreatedEvent {
    pub post_id: PostId,
    pub title: String,
    pub content: String,
    pub occurred_on: Date,
    pub version: i64,
}

impl IDomainEvent for PostCreatedEvent {
    fn event_type(&self) -> &'static str {
        "PostCreated"
    }
    fn event_id(&self) -> String {
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
