use blog_domain::value_objects::post_id::PostId;
use shared_kernel::application::ICommand;

#[derive(Debug, Clone)]
pub struct PublishPostCommand(pub String);

impl ICommand for PublishPostCommand {}

impl From<PublishPostCommand> for PostId {
    fn from(value: PublishPostCommand) -> Self {
        Self::new(&value.0)
    }
}
