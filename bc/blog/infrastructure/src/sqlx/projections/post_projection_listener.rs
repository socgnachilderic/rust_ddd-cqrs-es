use std::sync::Arc;

use anyhow::Ok;
use async_trait::async_trait;
use blog_domain::events::{PostAggregateEvent, PostContentEdited, PostCreated, PostTitleChanged};
use chrono::DateTime;
use shared_kernel::{application::events::IEventListener, domain::domain_event::IDomainEvent};
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct PostProjectionListener {
    pool: Arc<PgPool>,
}

impl PostProjectionListener {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    async fn handle_post_created_event(&self, event: &PostCreated) -> anyhow::Result<()> {
        let occurred_on = DateTime::parse_from_rfc3339(&event.occurred_on().to_iso8601())?;

        sqlx::query(
            "
            INSERT INTO post_projection (id, title, content, version, created_on, updated_on) 
            VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(event.aggregate_id.to_string())
        .bind(&event.title)
        .bind(&event.content)
        .bind(event.version)
        .bind(occurred_on)
        .bind(occurred_on)
        .execute(self.pool.as_ref())
        .await?;

        Ok(())
    }

    async fn handle_post_title_changed_event(
        &self,
        event: &PostTitleChanged,
    ) -> anyhow::Result<()> {
        let occurred_on = DateTime::parse_from_rfc3339(&event.occurred_on().to_iso8601())?;

        sqlx::query(
            "
            UPDATE post_projection 
            SET title = $2, version = $3, updated_on = $4
            WHERE id = $1
            ",
        )
        .bind(event.aggregate_id.to_string())
        .bind(&event.title)
        .bind(event.version)
        .bind(occurred_on)
        .execute(self.pool.as_ref())
        .await?;

        Ok(())
    }

    async fn handle_post_content_edited_event(
        &self,
        event: &PostContentEdited,
    ) -> anyhow::Result<()> {
        let occurred_on = DateTime::parse_from_rfc3339(&event.occurred_on().to_iso8601())?;

        sqlx::query(
            "
            UPDATE post_projection
            SET content = $2, version = $3, updated_on = $4
            WHERE id = $1
            ",
        )
        .bind(event.aggregate_id.to_string())
        .bind(&event.content)
        .bind(event.version)
        .bind(occurred_on)
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}

#[async_trait]
impl IEventListener<PostAggregateEvent> for PostProjectionListener {
    async fn handle(
        &self,
        event: &PostAggregateEvent,
        last_processed_version: i64,
    ) -> anyhow::Result<()> {
        if event.version() <= last_processed_version {
            return Ok(());
        }

        match event {
            PostAggregateEvent::PostCreated(event) => self.handle_post_created_event(event).await,
            PostAggregateEvent::PostTitleChanged(event) => {
                self.handle_post_title_changed_event(event).await
            }
            PostAggregateEvent::PostContentEdited(event) => {
                self.handle_post_content_edited_event(event).await
            }
        }
    }
}
