use std::{any::Any, fmt::Debug};

use async_trait::async_trait;

use super::date::Date;

pub trait IDomainEvent: Any + Debug + Send + Sync {
    fn version(&self) -> i64;
    fn event_type(&self) -> &'static str;
    fn event_id(&self) -> String;
    fn occurred_on(&self) -> Date;
    fn as_any(&self) -> &dyn Any;
}

#[async_trait]
pub trait EventStore<T>: Send + Sync
where
    T: IDomainEvent,
{
    async fn append(&self, events: &T) -> anyhow::Result<()>;
    async fn get_events_for_aggregate(&self, aggregate_id: &str) -> anyhow::Result<Vec<T>>;
    async fn get_all_events_since(&self, occurred_on: Date) -> anyhow::Result<Vec<T>>;
}
