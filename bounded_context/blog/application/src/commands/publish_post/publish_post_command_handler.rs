use crate::commands::publish_post::publish_post_command::PublishPostCommand;
use blog_domain::repositories::post_repository::{IReadPostRepository, IWritePostRepository};
use shared_kernel::application::ICommandHandler;

pub struct PublishPostCommandHandler<W, R>
where
    W: IWritePostRepository,
    R: IReadPostRepository,
{
    write_post_repository: W,
    read_post_repository: R,
}

impl<W, R> ICommandHandler<PublishPostCommand, ()> for PublishPostCommandHandler<W, R>
where
    W: IWritePostRepository,
    R: IReadPostRepository,
{
    async fn execute(&self, command: PublishPostCommand) {
        if let Some(mut post) = self.read_post_repository.get(&command.into()).await {
            post.publish();
            self.write_post_repository.add(post).await;
        }
    }
}
