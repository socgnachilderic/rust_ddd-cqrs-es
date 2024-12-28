use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use shared_kernel::application::commands::{ICommand, ICommandBusMiddleware, ICommandHandler};

use crate::interfaces::ILogger;

pub struct LoggerMiddleware<T, C, R>
where
    T: ILogger,
    C: ICommand,
{
    logger: T,
    _c: PhantomData<C>,
    _r: PhantomData<R>,
}

impl<T, C, R> LoggerMiddleware<T, C, R>
where
    T: ILogger,
    C: ICommand,
{
    pub fn new(logger: T) -> Self {
        Self {
            logger,
            _c: PhantomData,
            _r: PhantomData,
        }
    }
}

#[async_trait]
impl<T, C, R> ICommandBusMiddleware<C, R> for LoggerMiddleware<T, C, R>
where
    T: ILogger,
    C: ICommand,
    R: Send + Sync,
{
    async fn process(&self, command: &C, next: Arc<dyn ICommandHandler<C, R>>) -> R {
        self.logger.info(&format!(
            "LoggingMiddleware: Received command of type {}",
            std::any::type_name::<C>()
        ));

        let response = next.execute(command).await;

        self.logger
            .info("LoggingMiddleware: Command processed successfully");

        response
    }
}
