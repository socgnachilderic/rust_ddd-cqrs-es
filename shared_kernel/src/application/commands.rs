use std::sync::Arc;

use async_trait::async_trait;

pub trait ICommand: Sync {}

pub trait ICommandResponse {}

#[async_trait]
pub trait ICommandHandler<T: ICommand, R> {
    async fn execute(&self, command: T) -> R;

    fn listen_to() -> &'static str;
}

pub trait ICommandBus {}

#[async_trait]
pub trait ICommandBusMiddleware: Sync + Send {
    async fn process(&self, command: &dyn ICommand) -> Arc<dyn ICommandResponse>;
}

// pub trait ICommandDispatcher: CommandBusMiddleware {
//     async fn dispatch(&self, command: &dyn ICommand) -> Box<dyn ICommandResponse> {
//         // let command_type = c;
//         let handlers = self.get_handler(command.listen_to());

//         handlers.execute(command).await
//     }

//     fn get_handler(&self, listen_to: &'static str) -> Box<dyn ICommandHandler<dyn ICommand, dyn ICommandResponse>>;
// }
