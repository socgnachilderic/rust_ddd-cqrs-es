use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use blog_application::commands::actions::{CreatePostCommand, UpdatePostCommand};
#[cfg(feature = "server")]
use blog_domain::aggregate_root::Post as PostEntity;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
}

#[cfg(feature = "server")]
impl From<PostEntity> for Post {
    fn from(post: PostEntity) -> Self {
        Self {
            id: post.id.to_string(),
            title: post.title.to_string(),
            content: post.content.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CreatePostInput {
    pub title: String,
    pub content: String,
}

impl CreatePostInput {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
        }
    }
}

#[cfg(feature = "server")]
impl From<CreatePostInput> for CreatePostCommand {
    fn from(value: CreatePostInput) -> Self {
        Self {
            title: value.title,
            content: value.content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UpdatePostInput {
    pub post_id: String,
    pub title: Option<String>,
    pub content: Option<String>,
}

impl UpdatePostInput {
    pub fn with_id(mut self, id: &str) -> Self {
        self.post_id = id.to_string();
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn with_content(mut self, content: &str) -> Self {
        self.content = Some(content.to_string());
        self
    }
}

#[cfg(feature = "server")]
impl From<UpdatePostInput> for UpdatePostCommand {
    fn from(value: UpdatePostInput) -> Self {
        Self {
            post_id: value.post_id,
            title: value.title,
            content: value.content,
        }
    }
}
