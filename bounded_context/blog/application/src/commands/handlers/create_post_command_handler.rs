use async_trait::async_trait;
use blog_domain::{aggregate_root::Post, repositories::post_repository::IWritePostRepository};
use shared_kernel::application::commands::ICommandHandler;

use crate::commands::actions::CreatePostCommand;

pub struct CreatePostCommandHandler<T: IWritePostRepository> {
    write_post_repository: T,
}

impl<T: IWritePostRepository + Clone> CreatePostCommandHandler<T> {
    pub fn new(write_post_repository: &T) -> Self {
        Self {
            write_post_repository: write_post_repository.clone(),
        }
    }
}

#[async_trait]
impl<T: IWritePostRepository> ICommandHandler<CreatePostCommand, Post>
    for CreatePostCommandHandler<T>
{
    async fn execute(&self, command: &CreatePostCommand) -> Post {
        self.write_post_repository.add(command.into()).await
    }
}
