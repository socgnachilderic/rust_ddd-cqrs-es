use shared_kernel::application::ICommand;

#[derive(Debug, Clone)]
pub struct CommentPostComment {
    pub post_id: String,
    pub comment: String,
}

impl ICommand for CommentPostComment {}
