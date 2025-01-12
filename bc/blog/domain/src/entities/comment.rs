use crate::value_objects::comment_id::CommentId;
use shared_kernel::domain::Entity;

#[derive(Debug, Clone, Entity)]
pub struct Comment {
    pub id: CommentId,
    pub body: String,
}

impl Comment {
    pub fn new(id: CommentId, body: &str) -> Self {
        Self {
            id,
            body: body.to_owned(),
        }
    }
}

impl PartialEq for Comment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
