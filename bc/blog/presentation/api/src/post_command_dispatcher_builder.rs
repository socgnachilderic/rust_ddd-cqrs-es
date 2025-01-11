use std::sync::Arc;

use blog_application::commands::{
    handlers::{
        CommentPostCommandHandler, CreatePostCommandHandler, PublishPostCommandHandler,
        UpdatePostCommandHandler,
    },
    middlewares::LoggerMiddleware,
};
use blog_infrastructure::{
    chrono_date_provider::ChronoDateProvider,
    logger::SimpleLogger,
    sqlx::{
        projections::PostProjectionListener, sqlx_post_repository::SqlxPostRepository, Pool,
        SqlxEventStore,
    },
};
use shared_kernel::application::{
    commands::{CommandBusMiddlewareHandler, CommandDispatcher},
    events::EventDispatcher,
};

pub struct PostCommandDispatcherBuilder {
    command_dispatcher: CommandDispatcher,
    post_repository: SqlxPostRepository,
    date_provider: ChronoDateProvider,
    event_store: SqlxEventStore,
    post_projection_listener: Arc<PostProjectionListener>,
}

impl PostCommandDispatcherBuilder {
    pub fn new(pool: Arc<Pool>) -> Self {
        let date_provider = ChronoDateProvider;

        Self {
            command_dispatcher: CommandDispatcher::default(),
            post_repository: SqlxPostRepository::new(pool.clone()),
            post_projection_listener: Arc::new(PostProjectionListener::new(pool.clone())),
            event_store: SqlxEventStore::new(pool, &date_provider),
            date_provider,
        }
    }

    pub fn with_create_post_pipeline(mut self) -> Self {
        let event_dispatcher =
            EventDispatcher::default().register(self.post_projection_listener.clone());
        let logging_middleware = Arc::new(LoggerMiddleware::new(SimpleLogger));
        let create_post_handler = Arc::new(CreatePostCommandHandler::new(
            &self.event_store,
            &self.date_provider,
            event_dispatcher,
        ));
        let create_post_pipeline = CommandBusMiddlewareHandler::new(create_post_handler)
            .add_middleware(logging_middleware);

        self.command_dispatcher
            .register(Arc::new(create_post_pipeline));

        self
    }

    pub fn with_update_post_pipeline(mut self) -> Self {
        let event_dispatcher =
            EventDispatcher::default().register(self.post_projection_listener.clone());
        let logging_middleware = Arc::new(LoggerMiddleware::new(SimpleLogger));
        let update_post_handler = Arc::new(UpdatePostCommandHandler::new(
            &self.event_store,
            &self.date_provider,
            event_dispatcher,
        ));
        let update_post_pipeline = CommandBusMiddlewareHandler::new(update_post_handler)
            .add_middleware(logging_middleware);

        self.command_dispatcher
            .register(Arc::new(update_post_pipeline));

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

        self.command_dispatcher
            .register(Arc::new(publish_post_pipeline));

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

        self.command_dispatcher
            .register(Arc::new(comment_post_pipeline));

        self
    }

    pub fn build(self) -> CommandDispatcher {
        self.command_dispatcher
    }
}
