pub trait IQuery {}

impl IQuery for () {}

#[trait_variant::make(QueryHandler: Send)]
pub trait IQueryHandler<T: IQuery, R> {
    async fn execute(&self, query: T) -> R;
}

pub trait ICommand {}

impl ICommand for () {}

#[trait_variant::make(CommandHandler: Send)]
pub trait ICommandHandler<T: ICommand, R> {
    async fn execute(&self, command: T) -> R;
}
