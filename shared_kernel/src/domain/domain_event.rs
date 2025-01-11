use std::{any::Any, fmt::Debug};

use async_trait::async_trait;

use super::{date::Date, IAggregateRoot};

pub trait IDomainEvent: Any + Debug + Send + Sync {
    fn version(&self) -> i64;
    fn event_type(&self) -> &'static str;
    fn aggregate_id(&self) -> String;
    fn occurred_on(&self) -> Date;
    fn as_any(&self) -> &dyn Any;
}

pub trait IApplyDomainEvent<T: IAggregateRoot>: IDomainEvent {
    fn apply_to(&self, aggregate: &mut T);
}

pub trait IObservableAggregateRoot<T: IAggregateRoot>: IAggregateRoot + Send + Sync {
    type Event: IApplyDomainEvent<T>;

    fn clear_uncommitted_events(&mut self);
    fn get_uncommitted_events(&mut self) -> &[Self::Event];
    fn apply(&mut self, event: Self::Event);
}

#[async_trait]
pub trait EventStore<T>: Send + Sync
where
    T: IDomainEvent,
{
    async fn append(&self, events: &T) -> anyhow::Result<()>;
    async fn get_events_for_aggregate(&self, aggregate_id: &str) -> anyhow::Result<Vec<T>>;
    async fn load_events_after_version(
        &self,
        aggregate_id: &str,
        version: i64,
    ) -> anyhow::Result<Vec<T>>;
}
