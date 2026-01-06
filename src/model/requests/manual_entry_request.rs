use chrono::{DateTime, Utc};
use eywa_axum::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ManualEntryRequest {
    pub project_id: Option<Uuid>,
    pub tag_id: Option<Uuid>,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}
