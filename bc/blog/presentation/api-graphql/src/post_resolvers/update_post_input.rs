use async_graphql::InputObject;
use blog_application::commands::actions::UpdatePostCommand;

#[derive(InputObject)]
pub struct UpdatePostInput {
    pub post_id: String,
    pub title: Option<String>,
    pub content: Option<String>,
}

impl From<UpdatePostInput> for UpdatePostCommand {
    fn from(value: UpdatePostInput) -> Self {
        Self {
            post_id: value.post_id,
            title: value.title,
            content: value.content,
        }
    }
}
