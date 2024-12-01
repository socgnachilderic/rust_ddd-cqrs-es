use async_graphql::*;
use blog_domain::aggregate_root::Post;

#[derive(SimpleObject)]
pub(crate) struct PostObject {
    id: String,
    title: String,
    content: String,
}

impl From<Post> for PostObject {
    fn from(post: Post) -> Self {
        PostObject {
            id: post.id.to_string(),
            title: post.title.to_string(),
            content: post.content.to_string(),
        }
    }
}
