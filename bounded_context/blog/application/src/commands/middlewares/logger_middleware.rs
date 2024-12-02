use std::{any::type_name_of_val, sync::Arc};

use async_trait::async_trait;
use shared_kernel::application::commands::{ICommand, ICommandBusMiddleware, ICommandResponse};

use crate::interfaces::ILogger;

pub struct LoggerMiddleware<T: ILogger> {
    logger: T,
    next: Arc<dyn ICommandBusMiddleware>,
}

impl<T: ILogger> LoggerMiddleware<T> {
    pub fn new(next: Arc<dyn ICommandBusMiddleware>, logger: T) -> Self {
        LoggerMiddleware { logger, next }
    }
}

#[async_trait]
impl<T: ILogger> ICommandBusMiddleware for LoggerMiddleware<T> {
    async fn process(&self, command: &dyn ICommand) -> Arc<dyn ICommandResponse> {
        let response = self.next.process(command).await;

        self.logger.info(&format!(
            "Processing command: {}",
            type_name_of_val(command)
        ));

        response
    }
}
