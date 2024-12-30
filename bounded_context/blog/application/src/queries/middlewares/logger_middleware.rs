use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use shared_kernel::application::queries::{IQuery, IQueryBusMiddleware, IQueryHandler};

use crate::interfaces::ILogger;

pub struct LoggerMiddleware<T, Q, R>
where
    T: ILogger,
    Q: IQuery,
{
    logger: T,
    _c: PhantomData<Q>,
    _r: PhantomData<R>,
}

impl<T, Q, R> LoggerMiddleware<T, Q, R>
where
    T: ILogger,
    Q: IQuery,
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
impl<T, Q, R> IQueryBusMiddleware<Q, R> for LoggerMiddleware<T, Q, R>
where
    T: ILogger,
    Q: IQuery,
    R: Send + Sync,
{
    async fn process(&self, query: &Q, next: Arc<dyn IQueryHandler<Q, R>>) -> R {
        self.logger.info(&format!(
            "LoggingMiddleware: Received query of type {}",
            std::any::type_name::<Q>()
        ));

        let response = next.execute(query).await;

        self.logger
            .info("LoggingMiddleware: Query processed successfully");

        response
    }
}
