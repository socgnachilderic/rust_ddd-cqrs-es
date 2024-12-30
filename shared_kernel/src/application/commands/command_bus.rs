use std::sync::Arc;

use async_trait::async_trait;

use super::{ICommand, ICommandHandler};

#[async_trait]
pub trait ICommandBusMiddleware<C: ICommand, R>: Sync + Send {
    async fn process(&self, command: &C, next: Arc<dyn ICommandHandler<C, R>>) -> R;
}

pub struct CommandBusMiddlewareHandler<C: ICommand, R> {
    handler: Arc<dyn ICommandHandler<C, R>>,
    middlewares: Vec<Arc<dyn ICommandBusMiddleware<C, R>>>,
}

impl<C: ICommand, R> CommandBusMiddlewareHandler<C, R> {
    pub fn new(handler: Arc<dyn ICommandHandler<C, R>>) -> Self {
        Self {
            handler,
            middlewares: vec![],
        }
    }

    pub fn add_middleware(mut self, middleware: Arc<dyn ICommandBusMiddleware<C, R>>) -> Self {
        self.middlewares.push(middleware);
        self
    }
}

#[async_trait::async_trait]
impl<C: ICommand, R> ICommandHandler<C, R> for CommandBusMiddlewareHandler<C, R> {
    async fn execute(&self, command: &C) -> R {
        let mut current_handler = self.handler.clone();

        for middleware in self.middlewares.iter().rev() {
            let next_handler = self.handler.clone();
            let middleware = middleware.clone();

            current_handler = Arc::new(CommandBusMiddlewareWrapper {
                middleware,
                next_handler,
            });
        }

        current_handler.execute(command).await
    }
}

struct CommandBusMiddlewareWrapper<C: ICommand, R> {
    middleware: Arc<dyn ICommandBusMiddleware<C, R>>,
    next_handler: Arc<dyn ICommandHandler<C, R>>,
}

#[async_trait::async_trait]
impl<C: ICommand, R> ICommandHandler<C, R> for CommandBusMiddlewareWrapper<C, R> {
    async fn execute(&self, command: &C) -> R {
        self.middleware
            .process(command, self.next_handler.clone())
            .await
    }
}
