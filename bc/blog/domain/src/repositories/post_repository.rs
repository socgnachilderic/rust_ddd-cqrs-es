use async_trait::async_trait;

use crate::aggregate_root::Post;
use crate::value_objects::post_id::PostId;

#[async_trait]
pub trait IWritePostRepository: Sync + Send {
    async fn add(&self, post: Post) -> Post;
}

#[async_trait]
pub trait IReadPostRepository: Sync + Send {
    async fn get(&self, id: &PostId) -> Option<Post>;
    async fn all(&self) -> Vec<Post>;
}
