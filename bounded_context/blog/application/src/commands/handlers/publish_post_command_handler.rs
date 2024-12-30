use crate::commands::actions::PublishPostCommand;
use async_trait::async_trait;
use blog_domain::repositories::post_repository::{IReadPostRepository, IWritePostRepository};
use shared_kernel::application::commands::ICommandHandler;

pub struct PublishPostCommandHandler<W, R>
where
    W: IWritePostRepository,
    R: IReadPostRepository,
{
    write_post_repository: W,
    read_post_repository: R,
}

impl<W, R> PublishPostCommandHandler<W, R>
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
impl<W, R> ICommandHandler<PublishPostCommand, ()> for PublishPostCommandHandler<W, R>
where
    W: IWritePostRepository,
    R: IReadPostRepository,
{
    async fn execute(&self, command: &PublishPostCommand) {
        if let Some(mut post) = self.read_post_repository.get(&command.into()).await {
            post.publish();
            self.write_post_repository.add(post).await;
        }
    }
}
