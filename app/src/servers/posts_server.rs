use crate::models::{CreatePostInput, Post, UpdatePostInput};
use dioxus::prelude::*;

#[cfg(feature = "server")]
mod posts_server {
    use crate::models::Post;
    use crate::servers::posts_server::{CreatePostInput, UpdatePostInput};
    use blog_api::InjectionContainer;
    use blog_application::commands::actions::{CreatePostCommand, UpdatePostCommand};
    use blog_application::queries::actions::GetAllPostQuery;
    use blog_domain::aggregate_root::Post as PostEntity;
    use blog_domain::value_objects::post_id::PostId;

    pub async fn get_all_posts() -> Vec<Post> {
        let inject_api = InjectionContainer::new().await;

        inject_api
            .post_query_dispatcher
            .dispatch::<GetAllPostQuery, Vec<PostEntity>>(GetAllPostQuery)
            .await
            .unwrap()
            .into_iter()
            .map(Post::from)
            .collect()
    }

    pub async fn create_post(post: CreatePostInput) -> String {
        let inject_api = InjectionContainer::new().await;

        inject_api
            .post_command_dispatcher
            .dispatch::<CreatePostCommand, PostId>(post.into())
            .await
            .unwrap()
            .to_string()
    }

    pub async fn update_post(post: UpdatePostInput) -> String {
        let post_id = post.post_id.clone();
        let inject_api = InjectionContainer::new().await;

        inject_api
            .post_command_dispatcher
            .dispatch::<UpdatePostCommand, ()>(post.into())
            .await
            .unwrap();

        post_id
    }
}

#[server(GetAllPostsServer)]
pub async fn get_all_posts_server() -> Result<Vec<Post>, ServerFnError> {
    let posts = posts_server::get_all_posts().await;
    Ok(posts)
}

#[server(CreatePostServer)]
pub async fn create_post_server(input: CreatePostInput) -> Result<String, ServerFnError> {
    let post_id = posts_server::create_post(input).await;
    Ok(post_id)
}

#[server(UpdatePostServer)]
pub async fn update_post_server(input: UpdatePostInput) -> Result<String, ServerFnError> {
    let post_id = posts_server::update_post(input).await;
    Ok(post_id)
}
