use blog_domain::aggregate_root::Post;
use blog_domain::repositories::post_repository::IReadPostRepository;
use shared_kernel::application::IQueryHandler;

pub struct AllPostsQuery<R: IReadPostRepository> {
    read_post_repository: R,
}

impl<R: IReadPostRepository> IQueryHandler<(), Vec<Post>> for AllPostsQuery<R> {
    async fn execute(&self, _query: ()) -> Vec<Post> {
        self.read_post_repository.all().await
    }
}