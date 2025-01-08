use std::sync::Arc;

use anyhow::Ok;
use async_trait::async_trait;
use blog_domain::events::{PostAggregateEvent, PostCreatedEvent};
use shared_kernel::application::events::IEventListener;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct PostProjectionListener {
    pool: Arc<PgPool>,
}

impl PostProjectionListener {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    async fn handle_user_created_event(&self, event: &PostCreatedEvent) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO post_projection (id, title, content) VALUES ($1, $2, $3)")
            .bind(event.post_id.to_string())
            .bind(&event.title)
            .bind(&event.content)
            .execute(self.pool.as_ref())
            .await?;

        Ok(())
    }
}

#[async_trait]
impl IEventListener<PostAggregateEvent> for PostProjectionListener {
    async fn handle(&self, event: &PostAggregateEvent) -> anyhow::Result<()> {
        match event {
            PostAggregateEvent::PostCreated(event) => self.handle_user_created_event(event).await,
        }
    }
}
