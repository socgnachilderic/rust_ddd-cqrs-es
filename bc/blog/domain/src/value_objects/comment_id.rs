use shared_kernel::domain::IValueObject;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommentId(String);

impl CommentId {
    pub fn new(id: &str) -> Self {
        CommentId(id.to_string())
    }

    pub fn generate() -> Self {
        CommentId(Uuid::new_v4().to_string())
    }
}

impl IValueObject for CommentId {}

impl AsRef<str> for CommentId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for CommentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}
