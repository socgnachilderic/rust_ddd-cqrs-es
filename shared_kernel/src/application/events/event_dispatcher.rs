use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::domain_event::IDomainEvent;

use super::{AnyEventListener, IEventListener};

#[derive(Default)]
pub struct EventDispatcher {
    listeners: Vec<Arc<dyn AnyEventListener>>,
}

impl EventDispatcher {
    pub fn register<E>(mut self, handler: Arc<dyn IEventListener<E>>) -> Self
    where
        E: IDomainEvent + 'static,
    {
        let wrapper = EventListenerWrapper::new(handler);
        self.listeners.push(Arc::new(wrapper));
        self
    }

    pub async fn dispatch(&self, event: &dyn IDomainEvent) {
        for listener in &self.listeners {
            let _ = listener.handle_boxed(event).await;
        }
    }
}

struct EventListenerWrapper<E> {
    inner: Arc<dyn IEventListener<E>>,
}

impl<E> EventListenerWrapper<E> {
    pub fn new(inner: Arc<dyn IEventListener<E>>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl<E> AnyEventListener for EventListenerWrapper<E>
where
    E: IDomainEvent + 'static,
{
    async fn handle_boxed(&self, event: &dyn IDomainEvent) -> anyhow::Result<()> {
        if let Some(evt) = event.as_any().downcast_ref::<E>() {
            self.inner.handle(evt).await
        } else {
            Err(anyhow::anyhow!("Invalid event type"))
        }
    }
}
