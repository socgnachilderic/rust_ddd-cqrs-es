use async_graphql::*;
use blog_domain::{aggregate_root::Post, events::PostCreated};

#[derive(SimpleObject)]
pub(crate) struct PostObject {
    id: String,
    title: String,
    content: String,
}

impl From<Post> for PostObject {
    fn from(post: Post) -> Self {
        Self {
            id: post.id.to_string(),
            title: post.title.to_string(),
            content: post.content.to_string(),
        }
    }
}

impl From<PostCreated> for PostObject {
    fn from(event: PostCreated) -> Self {
        Self {
            id: event.aggregate_id.to_string(),
            title: event.title.to_string(),
            content: event.content.to_string(),
        }
    }
}
