use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use super::{IQuery, IQueryHandler};

#[async_trait]
trait AnyQueryHandler: Send + Sync {
    async fn handle_boxed(&self, query: Box<dyn IQuery>) -> anyhow::Result<Box<dyn Any + Send>>;
}

#[derive(Default)]
pub struct QueryDispatcher {
    handlers: HashMap<TypeId, Arc<dyn AnyQueryHandler>>,
}

impl QueryDispatcher {
    pub fn register<Q, R>(&mut self, handler: Arc<dyn IQueryHandler<Q, R>>)
    where
        Q: IQuery + 'static,
        R: 'static + Send + Sync,
    {
        let wrapper = QueryHandlerWrapper::new(handler);
        let type_id = TypeId::of::<Q>();

        self.handlers.insert(type_id, Arc::new(wrapper));
    }

    pub async fn dispatch<Q, R>(&self, query: Q) -> anyhow::Result<R>
    where
        Q: IQuery + 'static,
        R: 'static + Send + Sync,
    {
        let type_id = IQuery::type_id(&query);

        if let Some(handler) = self.handlers.get(&type_id) {
            let result = handler.handle_boxed(Box::new(query)).await?;
            let result_typed = *result
                .downcast::<R>()
                .map_err(|_| anyhow::anyhow!("Invalid result type"))?;

            Ok(result_typed)
        } else {
            Err(anyhow::anyhow!("No handler registered for query"))
        }
    }
}

struct QueryHandlerWrapper<Q, R> {
    inner: Arc<dyn IQueryHandler<Q, R>>,
}

impl<Q, R> QueryHandlerWrapper<Q, R> {
    fn new(inner: Arc<dyn IQueryHandler<Q, R>>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl<Q, R> AnyQueryHandler for QueryHandlerWrapper<Q, R>
where
    Q: IQuery + 'static,
    R: Send + Sync + 'static,
{
    async fn handle_boxed(&self, query: Box<dyn IQuery>) -> anyhow::Result<Box<dyn Any + Send>> {
        if let Some(cmd) = query.as_any().downcast_ref::<Q>() {
            let response = self.inner.execute(cmd).await;
            Ok(Box::new(response))
        } else {
            Err(anyhow::anyhow!("Invalid query type"))
        }
    }
}
