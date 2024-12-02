use std::any::type_name;

use crate::commands::actions::UpdatePostCommand;
use async_trait::async_trait;
use blog_domain::repositories::post_repository::{IReadPostRepository, IWritePostRepository};
use blog_domain::value_objects::post_id::PostId;
use shared_kernel::application::commands::ICommandHandler;

pub struct UpdatePostCommandHandler<W, R>
where
    W: IWritePostRepository,
    R: IReadPostRepository,
{
    write_post_repository: W,
    read_post_repository: R,
}

#[async_trait]
impl<W, R> ICommandHandler<UpdatePostCommand, ()> for UpdatePostCommandHandler<W, R>
where
    W: IWritePostRepository,
    R: IReadPostRepository,
{
    async fn execute(&self, command: UpdatePostCommand) {
        let post_id = PostId::new(&command.post_id);
        if let Some(mut post) = self.read_post_repository.get(&post_id).await {
            if let Some(title) = command.title {
                post.change_title(&title);
            }

            if let Some(content) = command.content {
                post.edit_content(&content);
            }

            self.write_post_repository.add(post).await;
        }
    }

    fn listen_to() -> &'static str {
        type_name::<UpdatePostCommand>()
    }
}