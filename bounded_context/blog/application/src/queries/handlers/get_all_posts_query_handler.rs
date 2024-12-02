use async_trait::async_trait;
use blog_domain::aggregate_root::Post;
use blog_domain::repositories::post_repository::IReadPostRepository;
use shared_kernel::application::queries::IQueryHandler;

use crate::queries::actions::GetAllPostQuery;

pub struct GetAllPostsQueryHandler<R: IReadPostRepository> {
    read_post_repository: R,
}

impl<R: IReadPostRepository + Clone> GetAllPostsQueryHandler<R> {
    pub fn new(read_post_repository: &R) -> Self {
        Self {
            read_post_repository: read_post_repository.clone(),
        }
    }
}

#[async_trait]
impl<R: IReadPostRepository> IQueryHandler<GetAllPostQuery, Vec<Post>>
    for GetAllPostsQueryHandler<R>
{
    async fn execute(&self, _query: GetAllPostQuery) -> Vec<Post> {
        self.read_post_repository.all().await
    }
}
