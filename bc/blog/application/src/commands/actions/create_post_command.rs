use blog_domain::aggregate_root::Post;
use shared_kernel::application::commands::Command;

#[derive(Debug, Clone, Command)]
pub struct CreatePostCommand {
    pub title: String,
    pub content: String,
}

impl From<&CreatePostCommand> for Post {
    fn from(value: &CreatePostCommand) -> Self {
        Self::new(&value.title, &value.content)
    }
}
