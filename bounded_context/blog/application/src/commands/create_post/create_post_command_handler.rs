use crate::commands::create_post::create_post_command::CreatePostCommand;
use blog_domain::repositories::post_repository::IWritePostRepository;
use shared_kernel::application::ICommandHandler;

pub struct CreatePostCommandHandler<T: IWritePostRepository> {
    post_repository: T,
}

impl<T: IWritePostRepository> ICommandHandler<CreatePostCommand, ()> for CreatePostCommandHandler<T> {
    async fn execute(&self, command: CreatePostCommand) {
        self.post_repository.add(command.into()).await
    }
}
