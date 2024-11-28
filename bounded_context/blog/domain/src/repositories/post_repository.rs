use crate::aggregate_root::Post;
use crate::value_objects::post_id::PostId;

#[trait_variant::make(WritePostRepository: Send)]
pub trait IWritePostRepository {
    async fn add(&self, post: Post);
}

#[trait_variant::make(ReadPostRepository: Send)]
pub trait IReadPostRepository {
    async fn get(&self, id: &PostId) -> Option<Post>;
    async fn all(&self) -> Vec<Post>;
}
