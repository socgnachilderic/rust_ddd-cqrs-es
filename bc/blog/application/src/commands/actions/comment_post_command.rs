use shared_kernel::application::commands::ICommand;

#[derive(Debug, Clone)]
pub struct CommentPostCommand {
    pub post_id: String,
    pub comment: String,
}

impl ICommand for CommentPostCommand {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
