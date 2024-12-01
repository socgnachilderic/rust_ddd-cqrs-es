use blog_domain::aggregate_root::Post;
use blog_domain::value_objects::post_id::PostId;
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct PostModel {
    pub id: String,
    pub title: String,
    pub content: String,
    // pub state: String,
}

impl From<PostModel> for Post {
    fn from(val: PostModel) -> Self {
        Post::new(PostId::new(&val.id), &val.title, &val.content)
    }
}
