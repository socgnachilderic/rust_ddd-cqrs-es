use blog_domain::repositories::post_repository::{IReadPostRepository, IWritePostRepository};
use blog_domain::value_objects::post_id::PostId;
use shared_kernel::application::ICommandHandler;
use crate::commands::comment_post::comment_post_command::CommentPostComment;

pub struct CommentPostCommandHandler<W, R>
where
    W: IWritePostRepository,
    R: IReadPostRepository
{
    write_post_repository: W,
    read_post_repository: R,
}

impl<W, R> ICommandHandler<CommentPostComment, ()> for CommentPostCommandHandler<W, R>
where
    W: IWritePostRepository,
    R: IReadPostRepository
{
    async fn execute(&self, command: CommentPostComment) {
        let post_id = PostId::new(&command.post_id);

        if let Some(mut post) = self.read_post_repository.get(&post_id).await {
            post.comment(&command.comment);
            self.write_post_repository.add(post).await;
        }
    }
}