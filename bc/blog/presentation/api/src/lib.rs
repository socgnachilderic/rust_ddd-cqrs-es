pub mod post_command_dispatcher_builder;
pub mod post_query_dispatcher_builder;

use blog_infrastructure::sqlx::establish_connection;
use post_command_dispatcher_builder::PostCommandDispatcherBuilder;
use post_query_dispatcher_builder::PostQueryDispatcherBuilder;
use shared_kernel::application::{commands::CommandDispatcher, queries::QueryDispatcher};

pub struct InjectionContainer {
    pub post_command_dispatcher: CommandDispatcher,
    pub post_query_dispatcher: QueryDispatcher,
}

impl InjectionContainer {
    pub async fn new() -> Self {
        let pool = establish_connection().await.unwrap();

        Self {
            post_command_dispatcher: PostCommandDispatcherBuilder::new(pool.clone())
                .with_create_post_pipeline()
                .with_update_post_pipeline()
                .with_publish_post_pipeline()
                .with_comment_post_pipeline()
                .build(),

            post_query_dispatcher: PostQueryDispatcherBuilder::new(pool.clone())
                .with_get_post_pipeline()
                .with_get_all_posts_pipeline()
                .build(),
        }
    }
}
