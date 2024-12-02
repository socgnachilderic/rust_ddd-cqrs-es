use blog_application::commands::handlers::{
    CommentPostCommandHandler, CreatePostCommandHandler, PublishPostCommandHandler,
};
use blog_application::queries::handlers::{GetAllPostsQueryHandler, GetPostQueryHandler};
use blog_infrastructure::sqlx::{establish_connection, sqlx_post_repository::SqlxPostRepository};

pub struct InjectionContainer {
    pub get_all_posts_handler: GetAllPostsQueryHandler<SqlxPostRepository>,
    pub get_post_handler: GetPostQueryHandler<SqlxPostRepository>,
    pub create_post_handler: CreatePostCommandHandler<SqlxPostRepository>,
    pub publish_post_handler: PublishPostCommandHandler<SqlxPostRepository, SqlxPostRepository>,
    pub comment_post_handler: CommentPostCommandHandler<SqlxPostRepository, SqlxPostRepository>,
}

impl InjectionContainer {
    pub async fn new() -> Self {
        let pool = establish_connection().await.unwrap();
        let post_repository = SqlxPostRepository::new(pool);

        Self {
            get_all_posts_handler: GetAllPostsQueryHandler::new(&post_repository),
            get_post_handler: GetPostQueryHandler::new(&post_repository),
            create_post_handler: CreatePostCommandHandler::new(&post_repository),
            publish_post_handler: PublishPostCommandHandler::new(
                &post_repository,
                &post_repository,
            ),
            comment_post_handler: CommentPostCommandHandler::new(
                &post_repository,
                &post_repository,
            ),
        }
    }
}
