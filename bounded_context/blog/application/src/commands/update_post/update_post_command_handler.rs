use crate::commands::update_post::update_post_command::UpdatePostCommand;
use blog_domain::repositories::post_repository::{IWritePostRepository, ReadPostRepository};
use blog_domain::value_objects::post_id::PostId;
use shared_kernel::application::ICommandHandler;

pub struct UpdatePostCommandHandler<W, R>
where
    W: IWritePostRepository,
    R: ReadPostRepository,
{
    write_post_repository: W,
    read_post_repository: R,
}

impl<W, R> ICommandHandler<UpdatePostCommand, ()> for UpdatePostCommandHandler<W, R>
where
    W: IWritePostRepository,
    R: ReadPostRepository,
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
}
