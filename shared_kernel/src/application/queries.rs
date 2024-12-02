use async_trait::async_trait;

pub trait IQuery {}

#[async_trait]
pub trait IQueryHandler<T: IQuery, R> {
    async fn execute(&self, query: T) -> R;
}
