use std::sync::Arc;

use blog_application::queries::{
    handlers::{GetAllPostsQueryHandler, GetPostQueryHandler},
    middlewares::LoggerMiddleware,
};
use blog_infrastructure::{
    logger::SimpleLogger,
    sqlx::{sqlx_post_repository::SqlxPostRepository, Pool},
};
use shared_kernel::application::queries::{QueryBusMiddlewareHandler, QueryDispatcher};

pub struct PostQueryDispatcherBuilder {
    dispatcher: QueryDispatcher,
    post_repository: SqlxPostRepository,
}

impl PostQueryDispatcherBuilder {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self {
            dispatcher: QueryDispatcher::default(),
            post_repository: SqlxPostRepository::new(pool),
        }
    }

    pub fn with_get_post_pipeline(mut self) -> Self {
        let logging_middleware = Arc::new(LoggerMiddleware::new(SimpleLogger));
        let get_post_handler = Arc::new(GetPostQueryHandler::new(&self.post_repository));
        let get_post_pipeline =
            QueryBusMiddlewareHandler::new(get_post_handler).add_middleware(logging_middleware);

        self.dispatcher.register(Arc::new(get_post_pipeline));

        self
    }

    pub fn with_get_all_posts_pipeline(mut self) -> Self {
        let logging_middleware = Arc::new(LoggerMiddleware::new(SimpleLogger));
        let get_post_handler = Arc::new(GetAllPostsQueryHandler::new(&self.post_repository));
        let get_post_pipeline =
            QueryBusMiddlewareHandler::new(get_post_handler).add_middleware(logging_middleware);

        self.dispatcher.register(Arc::new(get_post_pipeline));

        self
    }

    pub fn build(self) -> QueryDispatcher {
        self.dispatcher
    }
}
