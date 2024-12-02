mod create_post_input;
mod post_object;

use async_graphql::{Context, Object};
use blog_api::InjectionContainer;
use blog_application::queries::actions::GetAllPostQuery;
use create_post_input::CreatePostInput;
use post_object::PostObject;
use shared_kernel::application::{commands::ICommandHandler, queries::IQueryHandler};

#[derive(Default)]
pub(crate) struct PostQuery;

#[Object]
impl PostQuery {
    async fn get_all_posts(&self, ctx: &Context<'_>) -> Vec<PostObject> {
        ctx.data_unchecked::<InjectionContainer>()
            .get_all_posts_handler
            .execute(GetAllPostQuery)
            .await
            .into_iter()
            .map(PostObject::from)
            .collect()
    }

    async fn get_post(&self, ctx: &Context<'_>, id: String) -> Option<PostObject> {
        ctx.data_unchecked::<InjectionContainer>()
            .get_post_handler
            .execute(id.into())
            .await
            .map(PostObject::from)
    }
}

#[derive(Default)]
pub(crate) struct PostMutation;

#[Object]
impl PostMutation {
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        create_post_input: CreatePostInput,
    ) -> PostObject {
        ctx.data_unchecked::<InjectionContainer>()
            .create_post_handler
            .execute(create_post_input.into())
            .await
            .into()
    }
}
