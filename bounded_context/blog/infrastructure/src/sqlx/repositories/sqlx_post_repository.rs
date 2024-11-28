use blog_domain::aggregate_root::Post;
use blog_domain::repositories::post_repository::{IReadPostRepository, IWritePostRepository};
use blog_domain::value_objects::post_id::PostId;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub struct SqlxPostRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> SqlxPostRepository<'a> {
    pub async fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
}

impl<'a> IWritePostRepository for SqlxPostRepository<'a> {
    async fn add(&self, post: Post) {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("")
            .await
            .unwrap();
        todo!()
    }
}

impl<'a> IReadPostRepository for SqlxPostRepository<'a> {
    async fn get(&self, id: &PostId) -> Option<Post> {
        todo!()
    }

    async fn all(&self) -> Vec<Post> {
        todo!()
    }
}
