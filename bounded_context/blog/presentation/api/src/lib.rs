use blog_application::{
    commands::{
        comment_post::comment_post_command_handler::CommentPostCommandHandler,
        create_post::create_post_command_handler::CreatePostCommandHandler,
        publish_post::publish_post_command_handler::PublishPostCommandHandler,
    },
    queries::{
        all_posts::all_posts_query_handler::AllPostsQueryHandler,
        get_post::get_post_query_handler::GetPostQueryHandler,
    },
};
use blog_infrastructure::sqlx::{establish_connection, sqlx_post_repository::SqlxPostRepository};

pub struct InjectionContainer {
    pub get_all_posts_handler: AllPostsQueryHandler<SqlxPostRepository>,
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
            get_all_posts_handler: AllPostsQueryHandler::new(&post_repository),
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
