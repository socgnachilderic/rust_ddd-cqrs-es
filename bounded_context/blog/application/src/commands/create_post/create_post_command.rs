use blog_domain::aggregate_root::Post;
use blog_domain::value_objects::post_id::PostId;
use shared_kernel::application::ICommand;

#[derive(Debug, Clone)]
pub struct CreatePostCommand {
    pub title: String,
    pub content: String,
}

impl ICommand for CreatePostCommand {}

impl From<CreatePostCommand> for Post {
    fn from(value: CreatePostCommand) -> Self {
        Self::new(PostId::generate(), &value.title, &value.content)
    }
}
