use blog_domain::value_objects::post_id::PostId;
use shared_kernel::application::commands::Command;

#[derive(Debug, Clone, Command)]
pub struct PublishPostCommand(pub String);

impl From<&PublishPostCommand> for PostId {
    fn from(value: &PublishPostCommand) -> Self {
        Self::new(&value.0)
    }
}
