use crate::value_objects::comment_id::CommentId;
use shared_kernel::domain::IEntity;

#[derive(Debug, Clone)]
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

impl IEntity for Comment {}

impl PartialEq for Comment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
