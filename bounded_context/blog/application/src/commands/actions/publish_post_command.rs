use blog_domain::value_objects::post_id::PostId;
use shared_kernel::application::commands::ICommand;

#[derive(Debug, Clone)]
pub struct PublishPostCommand(pub String);

impl ICommand for PublishPostCommand {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl From<&PublishPostCommand> for PostId {
    fn from(value: &PublishPostCommand) -> Self {
        Self::new(&value.0)
    }
}
