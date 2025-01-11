use std::sync::Arc;

use anyhow::Ok;
use async_trait::async_trait;
use blog_domain::{events::PostAggregateEvent, PostSnapshot};
use chrono::DateTime;
use shared_kernel::{
    domain::{
        date::IDateProvider,
        domain_event::{EventStore, IDomainEvent},
        snapshot::SnapshotRepository,
    },
    infrastructure::sqlx::{EventModel, SnapshotModel},
};
use sqlx::PgPool;

use crate::chrono_date_provider::ChronoDateProvider;

use super::payloads::PostAggregateEventPayload;

#[derive(Clone)]
pub struct SqlxEventStore {
    pool: Arc<PgPool>,
    date_provider: ChronoDateProvider,
}

impl SqlxEventStore {
    pub fn new(pool: Arc<PgPool>, date_provider: &ChronoDateProvider) -> Self {
        Self {
            pool,
            date_provider: date_provider.clone(),
        }
    }

    fn parse_model_events(&self, events: Vec<EventModel>) -> Vec<PostAggregateEvent> {
        events
            .iter()
            .map(|event| {
                let occurred_on = self
                    .date_provider
                    .parse(&event.occurred_on.to_rfc3339())
                    .unwrap();
                let payload: PostAggregateEventPayload =
                    serde_json::from_value(event.payload.clone()).unwrap();

                payload.into_event(event.version, occurred_on)
            })
            .collect::<Vec<PostAggregateEvent>>()
    }
}

#[async_trait]
impl SnapshotRepository<PostSnapshot> for SqlxEventStore {
    async fn take_snapshot(&self, snapshot: PostSnapshot) -> anyhow::Result<()> {
        let payload = serde_json::to_value(&snapshot).unwrap();

        sqlx::query(
            "
            INSERT INTO snapshot (aggregate_id, version, payload)
            VALUES ($1, $2, $3)
            ON CONFLICT (aggregate_id) DO UPDATE SET version = $2, payload = $3",
        )
        .bind(snapshot.id)
        .bind(snapshot.version)
        .bind(payload)
        .execute(self.pool.as_ref())
        .await?;

        Ok(())
    }

    async fn load_snapshot(&self, aggregate_id: &str) -> anyhow::Result<Option<PostSnapshot>> {
        let model =
            sqlx::query_as::<_, SnapshotModel>("SELECT * FROM snapshot WHERE aggregate_id = $1")
                .bind(aggregate_id)
                .fetch_optional(self.pool.as_ref())
                .await?;

        if let Some(model) = model {
            let payload: PostSnapshot = serde_json::from_value(model.payload).unwrap();
            Ok(Some(payload))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl EventStore<PostAggregateEvent> for SqlxEventStore {
    async fn append(&self, event: &PostAggregateEvent) -> anyhow::Result<()> {
        let payload = PostAggregateEventPayload::from(event);
        let payload = serde_json::to_value(payload).unwrap();
        let version = event.version();
        let occurred_on = DateTime::parse_from_rfc3339(&event.occurred_on().to_iso8601())?;

        sqlx::query(
            "INSERT INTO post_event_store (aggregate_id, version, event_type, payload, occurred_on)
            VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(event.aggregate_id())
        .bind(version)
        .bind(event.event_type())
        .bind(&payload)
        .bind(occurred_on)
        .execute(self.pool.as_ref())
        .await?;

        Ok(())
    }

    async fn get_events_for_aggregate(
        &self,
        aggregate_id: &str,
    ) -> anyhow::Result<Vec<PostAggregateEvent>> {
        let events = sqlx::query_as::<_, EventModel>(
            "
            SELECT * FROM post_event_store
            WHERE aggregate_id = $1
            ORDER BY version ASC",
        )
        .bind(aggregate_id)
        .fetch_all(self.pool.as_ref())
        .await?;

        Ok(self.parse_model_events(events))
    }

    async fn load_events_after_version(
        &self,
        aggregate_id: &str,
        version: i64,
    ) -> anyhow::Result<Vec<PostAggregateEvent>> {
        let events = sqlx::query_as::<_, EventModel>(
            "
            SELECT * FROM post_event_store
            WHERE aggregate_id = $1 AND version > $2
            ORDER BY version ASC",
        )
        .bind(aggregate_id)
        .bind(version)
        .fetch_all(self.pool.as_ref())
        .await?;

        Ok(self.parse_model_events(events))
    }
}
