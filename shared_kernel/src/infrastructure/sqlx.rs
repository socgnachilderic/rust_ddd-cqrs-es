use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotModel {
    pub aggregate_id: String,
    pub version: i64,
    pub payload: Value,
}

#[derive(FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct EventModel {
    pub aggregate_id: String,
    pub version: i64,
    pub payload: Value,
    pub occurred_on: DateTime<FixedOffset>,
}
