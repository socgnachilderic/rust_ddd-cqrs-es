use blog_domain::aggregate_root::Post;
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct PostModel {
    pub id: String,
    pub title: String,
    pub content: String,
    pub state: String,
}

impl From<Post> for PostModel {
    fn from(value: Post) -> Self {
        Self {
            id: value.id.to_string(),
            title: value.title,
            content: value.content,
            state: value.state.to_string(),
        }
    }
}
