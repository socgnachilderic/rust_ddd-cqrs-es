use shared_kernel::application::commands::Command;

#[derive(Debug, Clone, Command)]
pub struct CommentPostCommand {
    pub post_id: String,
    pub comment: String,
}
