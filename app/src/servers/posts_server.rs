use crate::models::{CreatePostInput, Post};
use dioxus::prelude::*;

#[cfg(feature = "server")]
mod posts_server {
    use crate::models::Post;
    use crate::servers::posts_server::CreatePostInput;
    use blog_api::InjectionContainer;
    use blog_application::commands::actions::CreatePostCommand;
    use blog_application::queries::actions::GetAllPostQuery;
    use blog_domain::aggregate_root::Post as PostEntity;

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

    pub async fn create_post(post: CreatePostInput) -> Post {
        let inject_api = InjectionContainer::new().await;

        inject_api
            .post_command_dispatcher
            .dispatch::<CreatePostCommand, PostEntity>(post.into())
            .await
            .unwrap()
            .into()
    }
}

#[server(GetAllPostsServer)]
pub async fn get_all_posts_server() -> Result<Vec<Post>, ServerFnError> {
    let posts = posts_server::get_all_posts().await;
    Ok(posts)
}

#[server(CreatePostServer)]
pub async fn create_post_server(input: CreatePostInput) -> Result<Post, ServerFnError> {
    let post = posts_server::create_post(input).await;

    Ok(post)
}
