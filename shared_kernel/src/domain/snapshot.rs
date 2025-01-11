use async_trait::async_trait;
use std::fmt::Debug;

pub trait ISnapshot: Debug + Send + Sync {}

#[async_trait]
pub trait SnapshotRepository<S: ISnapshot>: Sync + Send {
    async fn take_snapshot(&self, snapshot: S) -> anyhow::Result<()>;
    async fn load_snapshot(&self, aggregate_id: &str) -> anyhow::Result<Option<S>>;
}
