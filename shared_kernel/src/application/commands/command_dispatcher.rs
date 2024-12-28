use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use super::{ICommand, ICommandHandler};

#[async_trait]
trait AnyCommandHandler: Send + Sync {
    async fn handle_boxed(&self, command: Box<dyn ICommand>)
        -> anyhow::Result<Box<dyn Any + Send>>;
}

#[derive(Default)]
pub struct CommandDispatcher {
    handlers: HashMap<TypeId, Arc<dyn AnyCommandHandler>>,
}

impl CommandDispatcher {
    pub fn register<C, R>(&mut self, handler: Arc<dyn ICommandHandler<C, R>>)
    where
        C: ICommand + 'static,
        R: 'static + Send + Sync,
    {
        let wrapper = CommandHandlerWrapper::new(handler);
        let type_id = TypeId::of::<C>();

        self.handlers.insert(type_id, Arc::new(wrapper));
    }

    pub async fn dispatch<C, R>(&self, command: C) -> anyhow::Result<R>
    where
        C: ICommand + 'static,
        R: 'static + Send + Sync,
    {
        let type_id = ICommand::type_id(&command);

        if let Some(handler) = self.handlers.get(&type_id) {
            let result = handler.handle_boxed(Box::new(command)).await?;
            let result_typed = *result
                .downcast::<R>()
                .map_err(|_| anyhow::anyhow!("Invalid result type"))?;

            Ok(result_typed)
        } else {
            Err(anyhow::anyhow!("No handler registered for command"))
        }
    }
}

struct CommandHandlerWrapper<C, R> {
    inner: Arc<dyn ICommandHandler<C, R>>,
}

impl<C, R> CommandHandlerWrapper<C, R> {
    fn new(inner: Arc<dyn ICommandHandler<C, R>>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl<C, R> AnyCommandHandler for CommandHandlerWrapper<C, R>
where
    C: ICommand + 'static,
    R: Send + Sync + 'static,
{
    async fn handle_boxed(
        &self,
        command: Box<dyn ICommand>,
    ) -> anyhow::Result<Box<dyn Any + Send>> {
        if let Some(cmd) = command.as_any().downcast_ref::<C>() {
            let response = self.inner.execute(cmd).await;
            Ok(Box::new(response))
        } else {
            Err(anyhow::anyhow!("Invalid command type"))
        }
    }
}
