use async_graphql::InputObject;
use blog_application::commands::actions::CreatePostCommand;

#[derive(InputObject)]
pub struct CreatePostInput {
    pub title: String,
    pub content: String,
}

impl From<CreatePostInput> for CreatePostCommand {
    fn from(value: CreatePostInput) -> Self {
        Self {
            title: value.title,
            content: value.content,
        }
    }
}
