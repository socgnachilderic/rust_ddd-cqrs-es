mod create_post_input;
mod post_object;
mod update_post_input;

use async_graphql::{Context, Object};
use blog_api::InjectionContainer;
use blog_application::{
    commands::actions::{CreatePostCommand, UpdatePostCommand},
    queries::actions::{GetAllPostQuery, GetPostQuery},
};
use blog_domain::{aggregate_root::Post, value_objects::post_id::PostId};
use create_post_input::CreatePostInput;
use post_object::PostObject;
use update_post_input::UpdatePostInput;

#[derive(Default)]
pub(crate) struct PostQuery;

#[Object]
impl PostQuery {
    async fn get_all_posts(&self, ctx: &Context<'_>) -> Vec<PostObject> {
        ctx.data_unchecked::<InjectionContainer>()
            .post_query_dispatcher
            .dispatch::<GetAllPostQuery, Vec<Post>>(GetAllPostQuery)
            .await
            .unwrap()
            .into_iter()
            .map(PostObject::from)
            .collect()
    }

    async fn get_post(&self, ctx: &Context<'_>, id: String) -> Option<PostObject> {
        ctx.data_unchecked::<InjectionContainer>()
            .post_query_dispatcher
            .dispatch::<GetPostQuery, Option<Post>>(id.into())
            .await
            .unwrap()
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
        let injection_container = ctx.data_unchecked::<InjectionContainer>();

        let post_id = injection_container
            .post_command_dispatcher
            .dispatch::<CreatePostCommand, PostId>(create_post_input.into())
            .await
            .unwrap()
            .to_string();

        injection_container
            .post_query_dispatcher
            .dispatch::<GetPostQuery, Option<Post>>(GetPostQuery(post_id))
            .await
            .unwrap()
            .unwrap()
            .into()
    }

    async fn update_post(
        &self,
        ctx: &Context<'_>,
        update_post_input: UpdatePostInput,
    ) -> PostObject {
        let post_id = update_post_input.post_id.to_string();
        let injection_container = ctx.data_unchecked::<InjectionContainer>();

        injection_container
            .post_command_dispatcher
            .dispatch::<UpdatePostCommand, ()>(update_post_input.into())
            .await
            .unwrap();

        injection_container
            .post_query_dispatcher
            .dispatch::<GetPostQuery, Option<Post>>(GetPostQuery(post_id))
            .await
            .unwrap()
            .unwrap()
            .into()
    }
}
