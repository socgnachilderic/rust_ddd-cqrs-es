use std::any::{Any, TypeId};

use async_trait::async_trait;

mod query_bus;
mod query_dispatcher;
pub use query_bus::*;
pub use query_dispatcher::*;

pub trait IQuery: Any + Send + Sync {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn as_any(&self) -> &dyn Any;
}

#[async_trait]
pub trait IQueryHandler<Q: IQuery, R>: Send + Sync {
    async fn execute(&self, query: &Q) -> R;
}
