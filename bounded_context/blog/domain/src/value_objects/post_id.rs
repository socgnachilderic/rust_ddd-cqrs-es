use shared_kernel::domain::IValueObject;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PostId(String);

impl PostId {
    pub fn new(id: &str) -> PostId {
        PostId(id.to_string())
    }

    pub fn generate() -> PostId {
        PostId(Uuid::new_v4().to_string())
    }
}

impl IValueObject for PostId {}

impl AsRef<str> for PostId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for PostId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}
