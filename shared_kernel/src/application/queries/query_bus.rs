use std::sync::Arc;

use async_trait::async_trait;

use super::{IQuery, IQueryHandler};

#[async_trait]
pub trait IQueryBusMiddleware<Q: IQuery, R>: Sync + Send {
    async fn process(&self, query: &Q, next: Arc<dyn IQueryHandler<Q, R>>) -> R;
}

pub struct QueryBusMiddlewareHandler<Q: IQuery, R> {
    handler: Arc<dyn IQueryHandler<Q, R>>,
    middlewares: Vec<Arc<dyn IQueryBusMiddleware<Q, R>>>,
}

impl<Q: IQuery, R> QueryBusMiddlewareHandler<Q, R> {
    pub fn new(handler: Arc<dyn IQueryHandler<Q, R>>) -> Self {
        Self {
            handler,
            middlewares: vec![],
        }
    }

    pub fn add_middleware(mut self, middleware: Arc<dyn IQueryBusMiddleware<Q, R>>) -> Self {
        self.middlewares.push(middleware);
        self
    }
}

#[async_trait::async_trait]
impl<Q: IQuery, R> IQueryHandler<Q, R> for QueryBusMiddlewareHandler<Q, R> {
    async fn execute(&self, query: &Q) -> R {
        let mut current_handler = self.handler.clone();

        for middleware in self.middlewares.iter().rev() {
            let next_handler = self.handler.clone();
            let middleware = middleware.clone();

            current_handler = Arc::new(QueryBusMiddlewareWrapper {
                middleware,
                next_handler,
            });
        }

        current_handler.execute(query).await
    }
}

struct QueryBusMiddlewareWrapper<Q: IQuery, R> {
    middleware: Arc<dyn IQueryBusMiddleware<Q, R>>,
    next_handler: Arc<dyn IQueryHandler<Q, R>>,
}

#[async_trait::async_trait]
impl<Q: IQuery, R> IQueryHandler<Q, R> for QueryBusMiddlewareWrapper<Q, R> {
    async fn execute(&self, query: &Q) -> R {
        self.middleware
            .process(query, self.next_handler.clone())
            .await
    }
}
