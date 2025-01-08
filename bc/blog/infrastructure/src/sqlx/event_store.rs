use std::sync::Arc;

use anyhow::Ok;
use async_trait::async_trait;
use blog_domain::events::PostAggregateEvent;
use chrono::DateTime;
use shared_kernel::domain::{
    date::Date,
    domain_event::{EventStore, IDomainEvent},
};
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct SqlxEventStore {
    pool: Arc<PgPool>,
}

impl SqlxEventStore {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EventStore<PostAggregateEvent> for SqlxEventStore {
    async fn append(&self, event: &PostAggregateEvent) -> anyhow::Result<()> {
        let payload = serde_json::to_value(event).unwrap();
        let version = event.version();
        let occured_on = DateTime::parse_from_rfc3339(&event.occurred_on().to_iso8601())?;

        sqlx::query(
            "INSERT INTO post_event_store (aggregate_id, version, event_type, payload, occurred_on)
            VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(event.event_id())
        .bind(version)
        .bind(event.event_type())
        .bind(&payload)
        .bind(occured_on)
        .execute(self.pool.as_ref())
        .await?;

        Ok(())
    }

    async fn get_events_for_aggregate(
        &self,
        _aggregate_id: &str,
    ) -> anyhow::Result<Vec<PostAggregateEvent>> {
        Ok(vec![])
    }

    async fn get_all_events_since(
        &self,
        _occurred_on: Date,
    ) -> anyhow::Result<Vec<PostAggregateEvent>> {
        Ok(vec![])
    }
}
