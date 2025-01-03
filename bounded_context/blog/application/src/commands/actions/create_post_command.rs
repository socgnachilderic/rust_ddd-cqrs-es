use std::any::type_name;

use blog_domain::aggregate_root::Post;
use blog_domain::value_objects::post_id::PostId;
use shared_kernel::application::commands::ICommand;

#[derive(Debug, Clone)]
pub struct CreatePostCommand {
    pub title: String,
    pub content: String,
}

impl CreatePostCommand {
    pub fn listen_to() -> String {
        type_name::<CreatePostCommand>().to_string()
    }
}

impl ICommand for CreatePostCommand {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl From<&CreatePostCommand> for Post {
    fn from(value: &CreatePostCommand) -> Self {
        Self::new(PostId::generate(), &value.title, &value.content)
    }
}
