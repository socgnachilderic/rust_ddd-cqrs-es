use std::any::{Any, TypeId};

use async_trait::async_trait;

mod command_bus;
mod command_dispatcher;
pub use command_bus::*;
pub use command_dispatcher::*;

pub trait ICommand: Any + Send + Sync {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn as_any(&self) -> &dyn Any;
}

#[async_trait]
pub trait ICommandHandler<C: ICommand, R>: Send + Sync {
    async fn execute(&self, command: &C) -> R;
}
