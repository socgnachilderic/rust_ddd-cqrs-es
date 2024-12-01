use std::ops::Deref;
use std::sync::Arc;

use crate::sqlx::models::post_model::PostModel;
use blog_domain::aggregate_root::Post;
use blog_domain::repositories::post_repository::{IReadPostRepository, IWritePostRepository};
use blog_domain::value_objects::post_id::PostId;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct SqlxPostRepository {
    pool: Arc<PgPool>,
}

impl SqlxPostRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

impl IWritePostRepository for SqlxPostRepository {
    async fn add(&self, post: Post) -> Post {
        sqlx::query_as::<_, PostModel>("INSERT INTO post (id, title, content) VALUES ($1, $2, $3) RETURNING *")
            .bind(post.id.to_string())
            .bind(post.title)
            .bind(post.content)
            .fetch_one(self.pool.deref())
            .await
            .unwrap()
            .into()
    }
}

impl IReadPostRepository for SqlxPostRepository {
    async fn get(&self, id: &PostId) -> Option<Post> {
        sqlx::query_as::<_, PostModel>("SELECT * FROM post WHERE id=$1")
            .bind(id.to_string())
            .fetch_one(self.pool.deref())
            .await
            .map(|post| post.into())
            .ok()
    }

    async fn all(&self) -> Vec<Post> {
        sqlx::query_as::<_, PostModel>("SELECT * FROM post")
            .fetch_all(self.pool.deref())
            .await
            .unwrap()
            .into_iter()
            .map(|it| it.into())
            .collect()
    }
}
