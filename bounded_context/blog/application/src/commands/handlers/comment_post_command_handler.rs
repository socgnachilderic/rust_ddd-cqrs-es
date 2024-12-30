use async_trait::async_trait;
use blog_domain::repositories::post_repository::{IReadPostRepository, IWritePostRepository};
use blog_domain::value_objects::post_id::PostId;
use shared_kernel::application::commands::ICommandHandler;

use crate::commands::actions::CommentPostCommand;

pub struct CommentPostCommandHandler<W, R>
where
    W: IWritePostRepository,
    R: IReadPostRepository,
{
    write_post_repository: W,
    read_post_repository: R,
}

impl<W, R> CommentPostCommandHandler<W, R>
where
    W: IWritePostRepository + Clone,
    R: IReadPostRepository + Clone,
{
    pub fn new(write_post_repository: &W, read_post_repository: &R) -> Self {
        Self {
            write_post_repository: write_post_repository.clone(),
            read_post_repository: read_post_repository.clone(),
        }
    }
}

#[async_trait]
impl<W, R> ICommandHandler<CommentPostCommand, ()> for CommentPostCommandHandler<W, R>
where
    W: IWritePostRepository,
    R: IReadPostRepository,
{
    async fn execute(&self, command: &CommentPostCommand) {
        let post_id = PostId::new(&command.post_id);

        if let Some(mut post) = self.read_post_repository.get(&post_id).await {
            post.comment(&command.comment);
            self.write_post_repository.add(post).await;
        }
    }
}
