use std::sync::Arc;

use blog_application::commands::{
    handlers::{
        CommentPostCommandHandler, CreatePostCommandHandler, PublishPostCommandHandler,
        UpdatePostCommandHandler,
    },
    middlewares::LoggerMiddleware,
};
use blog_infrastructure::{
    logger::SimpleLogger,
    sqlx::{sqlx_post_repository::SqlxPostRepository, Pool},
};
use shared_kernel::application::commands::{CommandBusMiddlewareHandler, CommandDispatcher};

pub struct PostCommandDispatcherBuilder {
    dispatcher: CommandDispatcher,
    post_repository: SqlxPostRepository,
}

impl PostCommandDispatcherBuilder {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self {
            dispatcher: CommandDispatcher::default(),
            post_repository: SqlxPostRepository::new(pool),
        }
    }

    pub fn with_create_post_pipeline(mut self) -> Self {
        let logging_middleware = Arc::new(LoggerMiddleware::new(SimpleLogger));
        let create_post_handler = Arc::new(CreatePostCommandHandler::new(&self.post_repository));
        let create_post_pipeline = CommandBusMiddlewareHandler::new(create_post_handler)
            .add_middleware(logging_middleware);

        self.dispatcher.register(Arc::new(create_post_pipeline));

        self
    }

    pub fn with_update_post_pipeline(mut self) -> Self {
        let logging_middleware = Arc::new(LoggerMiddleware::new(SimpleLogger));
        let update_post_handler = Arc::new(UpdatePostCommandHandler::new(
            &self.post_repository,
            &self.post_repository,
        ));
        let update_post_pipeline = CommandBusMiddlewareHandler::new(update_post_handler)
            .add_middleware(logging_middleware);

        self.dispatcher.register(Arc::new(update_post_pipeline));

        self
    }

    pub fn with_publish_post_pipeline(mut self) -> Self {
        let logging_middleware = Arc::new(LoggerMiddleware::new(SimpleLogger));
        let publish_post_handler = Arc::new(PublishPostCommandHandler::new(
            &self.post_repository,
            &self.post_repository,
        ));
        let publish_post_pipeline = CommandBusMiddlewareHandler::new(publish_post_handler)
            .add_middleware(logging_middleware);

        self.dispatcher.register(Arc::new(publish_post_pipeline));

        self
    }

    pub fn with_comment_post_pipeline(mut self) -> Self {
        let logging_middleware = Arc::new(LoggerMiddleware::new(SimpleLogger));
        let comment_post_handler = Arc::new(CommentPostCommandHandler::new(
            &self.post_repository,
            &self.post_repository,
        ));
        let comment_post_pipeline = CommandBusMiddlewareHandler::new(comment_post_handler)
            .add_middleware(logging_middleware);

        self.dispatcher.register(Arc::new(comment_post_pipeline));

        self
    }

    pub fn build(self) -> CommandDispatcher {
        self.dispatcher
    }
}
