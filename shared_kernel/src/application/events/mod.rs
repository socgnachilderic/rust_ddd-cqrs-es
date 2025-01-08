mod event_dispatcher;

use async_trait::async_trait;
pub use event_dispatcher::*;

use crate::domain::domain_event::IDomainEvent;

#[async_trait]
pub(super) trait AnyEventListener: Send + Sync {
    async fn handle_boxed(&self, event: &dyn IDomainEvent) -> anyhow::Result<()>;
}

#[async_trait]
pub trait IEventListener<E: IDomainEvent>: Send + Sync {
    async fn handle(&self, event: &E) -> anyhow::Result<()>;
}
