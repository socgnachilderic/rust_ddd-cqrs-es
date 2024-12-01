use crate::queries::get_post::get_post_query::GetPostQuery;
use blog_domain::aggregate_root::Post;
use blog_domain::repositories::post_repository::IReadPostRepository;
use blog_domain::value_objects::post_id::PostId;
use shared_kernel::application::IQueryHandler;

pub struct GetPostQueryHandler<R: IReadPostRepository> {
    read_post_repository: R,
}

impl<R: IReadPostRepository + Clone> GetPostQueryHandler<R> {
    pub fn new(read_post_repository: &R) -> Self {
        Self {
            read_post_repository: read_post_repository.clone(),
        }
    }
}

impl<R: IReadPostRepository> IQueryHandler<GetPostQuery, Option<Post>> for GetPostQueryHandler<R> {
    async fn execute(&self, query: GetPostQuery) -> Option<Post> {
        let post_id = PostId::new(&query.0);

        self.read_post_repository.get(&post_id).await
    }
}
