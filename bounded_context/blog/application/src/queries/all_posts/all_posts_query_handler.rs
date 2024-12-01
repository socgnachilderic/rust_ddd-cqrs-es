use blog_domain::aggregate_root::Post;
use blog_domain::repositories::post_repository::IReadPostRepository;
use shared_kernel::application::IQueryHandler;

pub struct AllPostsQueryHandler<R: IReadPostRepository> {
    read_post_repository: R,
}

impl<R: IReadPostRepository + Clone> AllPostsQueryHandler<R> {
    pub fn new(read_post_repository: &R) -> Self {
        Self {
            read_post_repository: read_post_repository.clone(),
        }
    }
}

impl<R: IReadPostRepository> IQueryHandler<(), Vec<Post>> for AllPostsQueryHandler<R> {
    async fn execute(&self, _query: ()) -> Vec<Post> {
        self.read_post_repository.all().await
    }
}
